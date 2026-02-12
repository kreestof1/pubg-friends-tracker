use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::handlers::{
    dashboard_handler,
    player_handler::{self, AppState},
};

pub fn create_api_routes() -> Router<AppState> {
    Router::new()
        // Dashboard
        .route("/dashboard", get(dashboard_handler::get_dashboard_stats))
        // Stats
        .route("/stats/clear-cache", post(player_handler::clear_all_stats_cache))
        // Players
        .route("/players", post(player_handler::create_player))
        .route("/players", get(player_handler::get_players))
        .route("/players/refresh-all", post(player_handler::refresh_all_players))
        .route("/players/:id", get(player_handler::get_player))
        .route("/players/:id/stats", get(player_handler::get_player_stats))
        .route("/players/:id/refresh", post(player_handler::refresh_player))
        .route("/players/:id", delete(player_handler::delete_player))
        .route("/players/:id/matches", get(player_handler::get_player_matches))
}
