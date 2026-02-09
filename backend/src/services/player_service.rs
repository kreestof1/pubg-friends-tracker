use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;

use crate::{
    db::{MongoDb, PlayerRepository},
    models::{Player, PlayerSummary},
    services::{PubgApiService, StatsService},
};

pub struct PlayerService {
    db: Arc<MongoDb>,
    pubg_api: Arc<PubgApiService>,
    stats_service: Arc<StatsService>,
}

impl PlayerService {
    pub fn new(
        db: Arc<MongoDb>,
        pubg_api: Arc<PubgApiService>,
        stats_service: Arc<StatsService>,
    ) -> Self {
        PlayerService {
            db,
            pubg_api,
            stats_service,
        }
    }

    pub async fn add_player(
        &self,
        name: &str,
        shard: &str,
    ) -> Result<Player, Box<dyn std::error::Error>> {
        // Check if player already exists
        let repo = PlayerRepository::new(self.db.players());
        
        // Fetch player from PUBG API
        let pubg_response = self.pubg_api.get_player_by_name(shard, name).await?;
        
        if pubg_response.data.is_empty() {
            return Err("Player not found".into());
        }

        let pubg_player = &pubg_response.data[0];
        let account_id = &pubg_player.id;

        // Check if already in database
        if let Some(existing) = repo.find_by_account_id(account_id).await? {
            tracing::info!("Player {} already exists", name);
            return Ok(existing);
        }

        // Create new player
        let mut player = Player::new(
            account_id.clone(),
            pubg_player.attributes.name.clone(),
            shard.to_string(),
        );

        // Get last 5 match IDs
        let match_ids: Vec<String> = pubg_player
            .relationships
            .matches
            .data
            .iter()
            .take(5)
            .map(|m| m.id.clone())
            .collect();

        player.last_matches = match_ids;
        player.last_refreshed_at = Some(Utc::now());

        // Save to database
        let created_player = repo.create(player).await?;
        
        tracing::info!(
            "Player {} added successfully with ID {}",
            name,
            created_player.id.as_ref().unwrap().to_hex()
        );

        Ok(created_player)
    }

    pub async fn get_all_players(&self) -> Result<Vec<Player>, mongodb::error::Error> {
        let repo = PlayerRepository::new(self.db.players());
        repo.find_all().await
    }

    pub async fn get_player(&self, id: &ObjectId) -> Result<Option<Player>, mongodb::error::Error> {
        let repo = PlayerRepository::new(self.db.players());
        repo.find_by_id(id).await
    }

    pub async fn refresh_player(
        &self,
        id: &ObjectId,
    ) -> Result<Player, Box<dyn std::error::Error>> {
        let repo = PlayerRepository::new(self.db.players());
        
        let player = repo
            .find_by_id(id)
            .await?
            .ok_or("Player not found")?;

        // Fetch updated data from PUBG API
        let pubg_response = self
            .pubg_api
            .get_player_by_name(&player.shard, &player.name)
            .await?;

        if pubg_response.data.is_empty() {
            return Err("Player not found in PUBG API".into());
        }

        let pubg_player = &pubg_response.data[0];

        // Update match list
        let match_ids: Vec<String> = pubg_player
            .relationships
            .matches
            .data
            .iter()
            .take(5)
            .map(|m| m.id.clone())
            .collect();

        let mut updated_player = player.clone();
        updated_player.last_matches = match_ids;
        updated_player.last_refreshed_at = Some(Utc::now());

        // Update in database
        repo.update(id, updated_player.clone()).await?;

        // Invalidate stats cache
        self.stats_service.invalidate_cache(id).await;

        tracing::info!("Player {} refreshed successfully", player.name);

        Ok(updated_player)
    }

    pub async fn delete_player(&self, id: &ObjectId) -> Result<(), mongodb::error::Error> {
        let repo = PlayerRepository::new(self.db.players());
        
        // Delete player stats
        let stats_repo = crate::db::StatsRepository::new(self.db.stats());
        stats_repo.delete_by_player(id).await?;

        // Delete player
        repo.delete(id).await?;

        // Invalidate cache
        self.stats_service.invalidate_cache(id).await;

        tracing::info!("Player with ID {} deleted", id.to_hex());

        Ok(())
    }

    pub async fn get_player_matches(
        &self,
        id: &ObjectId,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let repo = PlayerRepository::new(self.db.players());
        
        let player = repo
            .find_by_id(id)
            .await?
            .ok_or("Player not found")?;

        Ok(player.last_matches)
    }
}
