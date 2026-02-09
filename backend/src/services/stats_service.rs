use chrono::{DateTime, Duration, Utc};
use moka::future::Cache;
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;

use crate::{
    db::{MongoDb, StatsRepository},
    models::{PlayerStats, PubgMatchResponse},
};

pub struct StatsService {
    cache: Cache<String, PlayerStats>,
    db: Arc<MongoDb>,
}

impl StatsService {
    pub fn new(db: Arc<MongoDb>) -> Self {
        // LRU cache with 1000 entries, TTL of 1 hour
        let cache = Cache::builder()
            .max_capacity(1000)
            .time_to_live(std::time::Duration::from_secs(3600))
            .build();

        StatsService { cache, db }
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

        // Stats not in cache or expired, need to compute
        tracing::info!("Computing stats for player {} (not in cache)", player_id.to_hex());
        
        // Return empty stats as placeholder - computation will be done by PlayerService
        // when fetching matches from PUBG API
        let ttl_hours = match period {
            "7d" => 24,   // 1 day TTL for 7 days period
            "30d" => 72,  // 3 days TTL for 30 days period
            "90d" => 168, // 7 days TTL for 90 days period
            _ => 24,
        };

        let stats = PlayerStats::new(
            *player_id,
            period.to_string(),
            mode.to_string(),
            shard.to_string(),
            ttl_hours,
        );

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
                    continue; // Skip matches outside period
                }
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
        // Invalidate all cached entries for this player
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

        tracing::info!("Cache invalidated for player {}", player_id.to_hex());
    }
}
