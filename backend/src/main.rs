use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

// Modules (commented out until implemented)
// mod config;
// mod db;
// mod handlers;
// mod middleware;
// mod models;
// mod routes;
// mod services;
// mod utils;

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

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive()); // TODO: Configure CORS properly in production

    // Run the server
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();

    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}
