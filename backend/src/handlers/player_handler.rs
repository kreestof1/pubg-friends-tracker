use axum::{extract::{Path, Query, State}, http::StatusCode, Json};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use validator::Validate;

use crate::{
    models::{CreatePlayerRequest, PlayerResponse, StatsResponse},
    services::{PlayerService, StatsService},
};

pub type AppState = Arc<AppStateInner>;

#[derive(Clone)]
pub struct AppStateInner {
    pub player_service: Arc<PlayerService>,
    pub stats_service: Arc<StatsService>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// POST /api/players
pub async fn create_player(
    State(state): State<AppState>,
    Json(payload): Json<CreatePlayerRequest>,
) -> Result<(StatusCode, Json<PlayerResponse>), (StatusCode, Json<ErrorResponse>)> {
    // Validate input
    if let Err(e) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Validation error: {}", e),
            }),
        ));
    }

    match state
        .player_service
        .add_player(&payload.name, &payload.shard)
        .await
    {
        Ok(player) => {
            let response = PlayerResponse::from(player);
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to add player: {}", e),
            }),
        )),
    }
}

// GET /api/players
pub async fn get_players(
    State(state): State<AppState>,
) -> Result<Json<Vec<PlayerResponse>>, (StatusCode, Json<ErrorResponse>)> {
    match state.player_service.get_all_players().await {
        Ok(players) => {
            let responses: Vec<PlayerResponse> = players.into_iter().map(PlayerResponse::from).collect();
            Ok(Json(responses))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to fetch players: {}", e),
            }),
        )),
    }
}

// GET /api/players/:id
pub async fn get_player(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<PlayerResponse>, (StatusCode, Json<ErrorResponse>)> {
    let object_id = ObjectId::parse_str(&id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid player ID format".to_string(),
            }),
        )
    })?;

    match state.player_service.get_player(&object_id).await {
        Ok(Some(player)) => {
            let response = PlayerResponse::from(player);
            Ok(Json(response))
        }
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Player not found".to_string(),
            }),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to fetch player: {}", e),
            }),
        )),
    }
}

// POST /api/players/:id/refresh
pub async fn refresh_player(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<PlayerResponse>, (StatusCode, Json<ErrorResponse>)> {
    let object_id = ObjectId::parse_str(&id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid player ID format".to_string(),
            }),
        )
    })?;

    match state.player_service.refresh_player(&object_id).await {
        Ok(player) => {
            let response = PlayerResponse::from(player);
            Ok(Json(response))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to refresh player: {}", e),
            }),
        )),
    }
}

// DELETE /api/players/:id
pub async fn delete_player(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let object_id = ObjectId::parse_str(&id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid player ID format".to_string(),
            }),
        )
    })?;

    match state.player_service.delete_player(&object_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to delete player: {}", e),
            }),
        )),
    }
}

// GET /api/players/:id/matches
pub async fn get_player_matches(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<String>>, (StatusCode, Json<ErrorResponse>)> {
    let object_id = ObjectId::parse_str(&id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid player ID format".to_string(),
            }),
        )
    })?;

    match state.player_service.get_player_matches(&object_id).await {
        Ok(matches) => Ok(Json(matches)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to fetch matches: {}", e),
            }),
        )),
    }
}

#[derive(Debug, Deserialize)]
pub struct StatsQuery {
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

// GET /api/players/:id/stats?period=7d&mode=all&shard=steam
pub async fn get_player_stats(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<StatsQuery>,
) -> Result<Json<StatsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let object_id = ObjectId::parse_str(&id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid player ID format".to_string(),
            }),
        )
    })?;

    // Get player to verify it exists
    let _player = match state.player_service.get_player(&object_id).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "Player not found".to_string(),
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

    // Get or compute stats
    match state
        .stats_service
        .get_or_compute_stats(&object_id, &query.period, &query.mode, &query.shard)
        .await
    {
        Ok(stats) => Ok(Json(StatsResponse::from(stats))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to fetch stats: {}", e),
            }),
        )),
    }
}
