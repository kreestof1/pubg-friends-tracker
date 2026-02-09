use mongodb::{
    bson::doc,
    options::{ClientOptions, IndexOptions},
    Client, Collection, Database, IndexModel,
};
use std::sync::Arc;

use crate::models::{Player, PlayerStats};

#[derive(Clone)]
pub struct MongoDb {
    pub client: Client,
    pub database: Database,
}

impl MongoDb {
    pub async fn new(mongodb_uri: &str) -> Result<Self, mongodb::error::Error> {
        let mut client_options = ClientOptions::parse(mongodb_uri).await?;
        
        // Configuration du pool de connexions
        client_options.max_pool_size = Some(10);
        client_options.min_pool_size = Some(2);
        
        let client = Client::with_options(client_options)?;
        
        // Ping pour vérifier la connexion
        client
            .database("admin")
            .run_command(doc! { "ping": 1 }, None)
            .await?;
        
        tracing::info!("Successfully connected to MongoDB");
        
        let database = client.database("pubg_tracker");
        
        Ok(MongoDb { client, database })
    }

    pub fn players(&self) -> Collection<Player> {
        self.database.collection("players")
    }

    pub fn stats(&self) -> Collection<PlayerStats> {
        self.database.collection("player_stats")
    }

    pub async fn create_indexes(&self) -> Result<(), mongodb::error::Error> {
        tracing::info!("Creating MongoDB indexes...");

        // Index unique sur account_id dans players
        let players_collection = self.players();
        let account_id_index = IndexModel::builder()
            .keys(doc! { "account_id": 1 })
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .name("account_id_unique".to_string())
                    .build(),
            )
            .build();
        players_collection
            .create_index(account_id_index, None)
            .await?;

        // Index composé sur player_stats
        let stats_collection = self.stats();
        let stats_index = IndexModel::builder()
            .keys(doc! {
                "player_id": 1,
                "period": 1,
                "mode": 1,
                "shard": 1
            })
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .name("player_stats_composite".to_string())
                    .build(),
            )
            .build();
        stats_collection.create_index(stats_index, None).await?;

        // Index TTL sur expires_at pour auto-delete des stats expirées
        let ttl_index = IndexModel::builder()
            .keys(doc! { "expires_at": 1 })
            .options(
                IndexOptions::builder()
                    .expire_after(std::time::Duration::from_secs(0))
                    .name("stats_ttl".to_string())
                    .build(),
            )
            .build();
        stats_collection.create_index(ttl_index, None).await?;

        tracing::info!("MongoDB indexes created successfully");
        Ok(())
    }
}

pub type SharedMongoDb = Arc<MongoDb>;
