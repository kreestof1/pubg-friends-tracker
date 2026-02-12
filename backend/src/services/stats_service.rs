use chrono::{DateTime, Duration, Utc};
use moka::future::Cache;
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;

use crate::{
    db::{MongoDb, StatsRepository, PlayerRepository},
    models::{PlayerStats, PubgMatchResponse},
    services::PubgApiService,
};

pub struct StatsService {
    pub cache: Cache<String, PlayerStats>,
    pub db: Arc<MongoDb>,
    pubg_api: Arc<PubgApiService>,
}

impl StatsService {
    pub fn new(db: Arc<MongoDb>, pubg_api: Arc<PubgApiService>) -> Self {
        // LRU cache with 1000 entries, TTL of 1 hour
        let cache = Cache::builder()
            .max_capacity(1000)
            .time_to_live(std::time::Duration::from_secs(3600))
            .build();

        StatsService { cache, db, pubg_api }
    }

    #[tracing::instrument(skip(self), fields(player_id = %player_id.to_hex(), period = %period, mode = %mode, shard = %shard))]
    pub async fn get_or_compute_stats(
        &self,
        player_id: &ObjectId,
        period: &str,
        mode: &str,
        shard: &str,
    ) -> Result<PlayerStats, mongodb::error::Error> {
        let cache_key = format!("{}:{}:{}:{}", player_id.to_hex(), period, mode, shard);

        // Check memory cache
        if let Some(cached_stats) = self.cache.get(&cache_key).await {
            tracing::debug!("Stats found in memory cache");
            return Ok(cached_stats);
        }

        // Check database cache
        let repo = StatsRepository::new(self.db.stats());
        if let Some(db_stats) = repo.find_by_player(player_id, period, mode, shard).await? {
            // Check if not expired
            if db_stats.expires_at > Utc::now() {
                tracing::debug!("Stats found in database cache for {}", cache_key);
                self.cache.insert(cache_key, db_stats.clone()).await;
                return Ok(db_stats);
            }
        }

        // Stats not in cache or expired, need to compute from real PUBG API data
        tracing::info!("Computing stats from PUBG API for player {} (not in cache)", player_id.to_hex());
        
        // Fetch player to get match_ids and account_id
        let player_repo = PlayerRepository::new(self.db.players());
        let player = player_repo
            .find_by_id(player_id)
            .await?
            .ok_or_else(|| {
                mongodb::error::Error::custom(format!("Player not found: {}", player_id.to_hex()))
            })?;

        // Fetch match details from PUBG API
        let mut matches: Vec<PubgMatchResponse> = Vec::new();
        
        if !player.last_matches.is_empty() {
            tracing::info!("Fetching {} matches from PUBG API", player.last_matches.len());
            
            for match_id in &player.last_matches {
                match self.pubg_api.get_match(&player.shard, match_id).await {
                    Ok(match_data) => {
                        tracing::debug!("Successfully fetched match {}", match_id);
                        matches.push(match_data);
                    }
                    Err(e) => {
                        // Log the error but continue with other matches
                        tracing::warn!("Failed to fetch match {}: {}", match_id, e);
                    }
                }
            }
        }

        // If no matches were fetched, return error
        if matches.is_empty() {
            tracing::error!("No match data available for player {}", player_id.to_hex());
            return Err(mongodb::error::Error::custom(
                "No match data available for this player. Please ensure the player has recent matches."
            ));
        }

        tracing::info!("Computing stats from {} matches", matches.len());

        // Compute stats from fetched matches
        let mut stats = self.compute_stats_from_matches(&player.account_id, &matches, period);
        
        // Set the correct player_id, mode, and shard
        stats.player_id = *player_id;
        stats.mode = mode.to_string();
        stats.shard = shard.to_string();

        // Cache the stats
        self.cache.insert(cache_key.clone(), stats.clone()).await;
        
        // Save to database (async, don't wait)
        let db = self.db.clone();
        let stats_to_save = stats.clone();
        tokio::spawn(async move {
            let repo = StatsRepository::new(db.stats());
            if let Err(e) = repo.upsert(stats_to_save).await {
                tracing::error!("Failed to save stats to database: {}", e);
            }
        });

        tracing::info!("Stats computed successfully for player {}", player_id.to_hex());
        Ok(stats)
    }

    pub fn compute_stats_from_matches(
        &self,
        player_account_id: &str,
        matches: &[PubgMatchResponse],
        period: &str,
    ) -> PlayerStats {
        let now = Utc::now();
        let period_start = match period {
            "7d" => now - Duration::days(7),
            "30d" => now - Duration::days(30),
            "90d" => now - Duration::days(90),
            _ => now - Duration::days(7),
        };

        tracing::debug!(
            "Computing stats for period {} (from {} to {}), processing {} matches",
            period,
            period_start,
            now,
            matches.len()
        );

        let mut total_kills = 0;
        let mut total_deaths = 0;
        let mut total_damage = 0.0;
        let mut total_survival_time = 0.0;
        let mut top1_count = 0;
        let mut matches_in_period = 0;

        for match_data in matches {
            // Parse match date
            if let Ok(match_date) = DateTime::parse_from_rfc3339(&match_data.data.attributes.created_at) {
                let match_utc = match_date.with_timezone(&Utc);
                
                if match_utc < period_start {
                    tracing::debug!("Skipping match {} from {} (before period start)", match_data.data.id, match_utc);
                    continue; // Skip matches outside period
                }
                
                tracing::debug!("Including match {} from {} (within period)", match_data.data.id, match_utc);
            }

            // Find participant data for this player
            for included in &match_data.included {
                if let crate::models::PubgMatchIncluded::Participant { id: _, attributes } = included {
                    if attributes.stats.player_id == player_account_id {
                        matches_in_period += 1;
                        total_kills += attributes.stats.kills;
                        total_damage += attributes.stats.damage_dealt;
                        total_survival_time += attributes.stats.time_survived;
                        
                        // Check if won (win_place = 1)
                        if attributes.stats.win_place == 1 {
                            top1_count += 1;
                        }
                        
                        // Check if died (death_type != "alive")
                        if attributes.stats.death_type != "alive" {
                            total_deaths += 1;
                        }
                        
                        break;
                    }
                }
            }
        }

        let kd_ratio = if total_deaths > 0 {
            total_kills as f64 / total_deaths as f64
        } else {
            total_kills as f64
        };

        let win_rate = if matches_in_period > 0 {
            (top1_count as f64 / matches_in_period as f64) * 100.0
        } else {
            0.0
        };

        let ttl_hours = match period {
            "7d" => 24,
            "30d" => 72,
            "90d" => 168,
            _ => 24,
        };

        tracing::info!(
            "Stats computed for period {}: {} matches, {} kills, {} deaths, K/D: {:.2}, Win rate: {:.1}%",
            period,
            matches_in_period,
            total_kills,
            total_deaths,
            kd_ratio,
            win_rate
        );

        PlayerStats {
            id: None,
            player_id: ObjectId::new(),  // Will be set by caller
            period: period.to_string(),
            mode: "all".to_string(),     // Will be set by caller
            shard: "steam".to_string(),  // Will be set by caller
            kills: total_kills,
            deaths: total_deaths,
            kd_ratio,
            win_rate,
            damage_dealt: total_damage,
            survival_time: total_survival_time,
            top1_count,
            matches_played: matches_in_period,
            computed_at: now,
            expires_at: now + Duration::hours(ttl_hours),
        }
    }

    pub async fn save_stats(&self, stats: &PlayerStats) -> Result<(), mongodb::error::Error> {
        let cache_key = format!(
            "{}:{}:{}:{}",
            stats.player_id.to_hex(),
            stats.period,
            stats.mode,
            stats.shard
        );

        // Save to database
        let repo = StatsRepository::new(self.db.stats());
        repo.upsert(stats.clone()).await?;

        // Update memory cache
        self.cache.insert(cache_key, stats.clone()).await;

        tracing::info!(
            "Stats saved for player {} ({} period)",
            stats.player_id.to_hex(),
            stats.period
        );

        Ok(())
    }

    pub async fn invalidate_cache(&self, player_id: &ObjectId) {
        // Invalidate memory cache entries for this player
        let periods = ["7d", "30d", "90d"];
        let modes = ["solo", "duo", "squad", "all"];
        let shards = ["steam", "xbox", "psn"];

        for period in periods {
            for mode in modes {
                for shard in shards {
                    let cache_key = format!("{}:{}:{}:{}", player_id.to_hex(), period, mode, shard);
                    self.cache.invalidate(&cache_key).await;
                }
            }
        }

        // Also delete stats from MongoDB to force recomputation
        let repo = StatsRepository::new(self.db.stats());
        if let Err(e) = repo.delete_by_player(player_id).await {
            tracing::warn!("Failed to delete stats from database for player {}: {}", player_id.to_hex(), e);
        }

        tracing::info!("Cache and database stats invalidated for player {}", player_id.to_hex());
    }
}
