use mongodb::{
    bson::{doc, oid::ObjectId, to_bson},
    Collection,
};

use crate::models::{Player, PlayerStats};

pub struct PlayerRepository {
    collection: Collection<Player>,
}

impl PlayerRepository {
    pub fn new(collection: Collection<Player>) -> Self {
        PlayerRepository { collection }
    }

    pub async fn create(&self, player: Player) -> Result<Player, mongodb::error::Error> {
        let result = self.collection.insert_one(&player, None).await?;
        let mut created_player = player;
        created_player.id = Some(result.inserted_id.as_object_id().unwrap());
        Ok(created_player)
    }

    pub async fn find_by_id(&self, id: &ObjectId) -> Result<Option<Player>, mongodb::error::Error> {
        self.collection.find_one(doc! { "_id": id }, None).await
    }

    pub async fn find_by_account_id(
        &self,
        account_id: &str,
    ) -> Result<Option<Player>, mongodb::error::Error> {
        self.collection
            .find_one(doc! { "account_id": account_id }, None)
            .await
    }

    pub async fn find_all(&self) -> Result<Vec<Player>, mongodb::error::Error> {
        use futures::stream::TryStreamExt;
        
        let cursor = self.collection.find(None, None).await?;
        cursor.try_collect().await
    }

    pub async fn update(&self, id: &ObjectId, player: Player) -> Result<(), mongodb::error::Error> {
        let update_doc = doc! {
            "$set": {
                "name": player.name,
                "shard": player.shard,
                "last_matches": player.last_matches,
                "last_refreshed_at": player.last_refreshed_at,
                "summary": to_bson(&player.summary).unwrap_or(mongodb::bson::Bson::Null),
            }
        };
        
        self.collection
            .update_one(doc! { "_id": id }, update_doc, None)
            .await?;
        Ok(())
    }

    pub async fn delete(&self, id: &ObjectId) -> Result<(), mongodb::error::Error> {
        self.collection
            .delete_one(doc! { "_id": id }, None)
            .await?;
        Ok(())
    }
}

pub struct StatsRepository {
    collection: Collection<PlayerStats>,
}

impl StatsRepository {
    pub fn new(collection: Collection<PlayerStats>) -> Self {
        StatsRepository { collection }
    }

    pub async fn create(&self, stats: PlayerStats) -> Result<PlayerStats, mongodb::error::Error> {
        let result = self.collection.insert_one(&stats, None).await?;
        let mut created_stats = stats;
        created_stats.id = Some(result.inserted_id.as_object_id().unwrap());
        Ok(created_stats)
    }

    pub async fn find_by_player(
        &self,
        player_id: &ObjectId,
        period: &str,
        mode: &str,
        shard: &str,
    ) -> Result<Option<PlayerStats>, mongodb::error::Error> {
        self.collection
            .find_one(
                doc! {
                    "player_id": player_id,
                    "period": period,
                    "mode": mode,
                    "shard": shard
                },
                None,
            )
            .await
    }

    pub async fn upsert(&self, stats: PlayerStats) -> Result<(), mongodb::error::Error> {
        let filter = doc! {
            "player_id": stats.player_id,
            "period": &stats.period,
            "mode": &stats.mode,
            "shard": &stats.shard
        };

        let update = doc! {
            "$set": {
                "kills": stats.kills,
                "deaths": stats.deaths,
                "kd_ratio": stats.kd_ratio,
                "win_rate": stats.win_rate,
                "damage_dealt": stats.damage_dealt,
                "survival_time": stats.survival_time,
                "top1_count": stats.top1_count,
                "matches_played": stats.matches_played,
                "computed_at": stats.computed_at,
                "expires_at": stats.expires_at,
            }
        };

        self.collection
            .update_one(filter, update, None)
            .await?;
        Ok(())
    }

    pub async fn delete_by_player(&self, player_id: &ObjectId) -> Result<(), mongodb::error::Error> {
        self.collection
            .delete_many(doc! { "player_id": player_id }, None)
            .await?;
        Ok(())
    }
}
