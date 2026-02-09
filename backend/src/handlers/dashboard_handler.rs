use axum::{extract::{Query, State}, http::StatusCode, Json};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    handlers::player_handler::{AppState, ErrorResponse},
    models::{PlayerResponse, StatsResponse},
    services::StatsService,
};

#[derive(Debug, Deserialize)]
pub struct DashboardQuery {
    pub player_ids: String, // Comma-separated player IDs
    #[serde(default = "default_period")]
    pub period: String,
    #[serde(default = "default_mode")]
    pub mode: String,
    #[serde(default = "default_shard")]
    pub shard: String,
}

fn default_period() -> String {
    "7d".to_string()
}

fn default_mode() -> String {
    "all".to_string()
}

fn default_shard() -> String {
    "steam".to_string()
}

#[derive(Debug, Serialize)]
pub struct DashboardResponse {
    pub players: Vec<PlayerWithStats>,
}

#[derive(Debug, Serialize)]
pub struct PlayerWithStats {
    pub player: PlayerResponse,
    pub stats: StatsResponse,
}

// GET /api/dashboard?player_ids=id1,id2,id3&period=7d&mode=all&shard=steam
pub async fn get_dashboard_stats(
    State(state): State<AppState>,
    Query(query): Query<DashboardQuery>,
) -> Result<Json<DashboardResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Parse player IDs
    let player_ids: Result<Vec<ObjectId>, _> = query
        .player_ids
        .split(',')
        .map(|id| ObjectId::parse_str(id.trim()))
        .collect();

    let player_ids = player_ids.map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid player ID format".to_string(),
            }),
        )
    })?;

    if player_ids.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "At least one player ID is required".to_string(),
            }),
        ));
    }

    if player_ids.len() > 10 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Maximum 10 players can be compared".to_string(),
            }),
        ));
    }

    let mut players_with_stats = Vec::new();

    for player_id in player_ids {
        // Get player
        let player = match state.player_service.get_player(&player_id).await {
            Ok(Some(p)) => p,
            Ok(None) => {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse {
                        error: format!("Player {} not found", player_id.to_hex()),
                    }),
                ))
            }
            Err(e) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Failed to fetch player: {}", e),
                    }),
                ))
            }
        };

        // Get stats - create a new StatsService instance to access the method
        let stats_service = Arc::new(StatsService::new(state.player_service.db.clone()));
        let stats = match stats_service
            .get_or_compute_stats(&player_id, &query.period, &query.mode, &query.shard)
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Failed to fetch stats: {}", e),
                    }),
                ))
            }
        };

        players_with_stats.push(PlayerWithStats {
            player: PlayerResponse::from(player),
            stats: StatsResponse::from(stats),
        });
    }

    Ok(Json(DashboardResponse {
        players: players_with_stats,
    }))
}
