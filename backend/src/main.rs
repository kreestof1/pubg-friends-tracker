use axum::{routing::get, Router};
use std::{net::SocketAddr, sync::Arc};

// Modules
mod config;
mod db;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
// mod utils;

use config::Config;
use db::MongoDb;
use handlers::AppStateInner;
use middleware::{create_cors_layer, handle_errors, trace_request};
use routes::create_api_routes;
use services::{PlayerService, PubgApiService, StatsService};

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize tracing with environment-based log level
    let log_level = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string());
    
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .with_env_filter(log_level)
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

    let player_service = Arc::new(PlayerService::new(
        shared_db.clone(),
        pubg_api.clone(),
        stats_service.clone(),
    ));

    tracing::info!("All services initialized successfully");

    // Create application state
    let app_state = Arc::new(AppStateInner { 
        player_service,
        stats_service: stats_service.clone(),
    });

    // Build API routes
    let api_routes = create_api_routes();

    // Build our application with routes
    let cors_origin = std::env::var("CORS_ORIGIN").unwrap_or_else(|_| "*".to_string());
    
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api", api_routes)
        .with_state(app_state)
        .layer(axum::middleware::from_fn(trace_request))
        .layer(axum::middleware::from_fn(handle_errors))
        .layer(create_cors_layer(&cors_origin));

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
