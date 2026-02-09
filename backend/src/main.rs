use axum::{routing::get, Router};
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::CorsLayer;

// Modules
mod config;
mod db;
mod models;
mod services;
// mod handlers;
// mod middleware;
// mod routes;
// mod utils;

use config::Config;
use db::MongoDb;
use services::{PlayerService, PubgApiService, StatsService};

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .json()
        .init();

    tracing::info!("Starting PUBG Tracker API...");

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    tracing::info!("Configuration loaded successfully");

    // Initialize MongoDB connection
    let mongodb = match MongoDb::new(&config.mongodb_uri).await {
        Ok(db) => {
            tracing::info!("Successfully connected to MongoDB");
            db
        }
        Err(e) => {
            tracing::error!("Failed to connect to MongoDB: {}", e);
            tracing::error!("Please ensure MongoDB is running:");
            tracing::error!("  - Docker: docker-compose up -d mongo");
            tracing::error!("  - Or ensure MongoDB is accessible at: {}", config.mongodb_uri);
            std::process::exit(1);
        }
    };

    // Create indexes
    if let Err(e) = mongodb.create_indexes().await {
        tracing::error!("Failed to create MongoDB indexes: {}", e);
        std::process::exit(1);
    }

    let shared_db = Arc::new(mongodb);

    // Initialize services
    let pubg_api = Arc::new(PubgApiService::new(
        config.pubg_api_key.clone(),
        config.pubg_api_base_url.clone(),
    ));

    let stats_service = Arc::new(StatsService::new(shared_db.clone()));

    let _player_service = Arc::new(PlayerService::new(
        shared_db.clone(),
        pubg_api.clone(),
        stats_service.clone(),
    ));

    tracing::info!("All services initialized successfully");

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive()); // TODO: Configure CORS properly in production

    // Run the server
    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .expect("Invalid address");

    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}
