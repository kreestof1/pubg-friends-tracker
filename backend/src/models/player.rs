use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub account_id: String,
    pub name: String,
    pub shard: String,
    #[serde(default)]
    pub last_matches: Vec<String>,
    pub last_refreshed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<PlayerSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSummary {
    pub total_matches: i32,
    pub total_kills: i32,
    pub total_deaths: i32,
    pub kd_ratio: f64,
    pub win_rate: f64,
    pub avg_damage: f64,
}

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct CreatePlayerRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 1))]
    pub shard: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerResponse {
    pub id: String,
    pub account_id: String,
    pub name: String,
    pub shard: String,
    pub last_refreshed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub summary: Option<PlayerSummary>,
}

impl From<Player> for PlayerResponse {
    fn from(player: Player) -> Self {
        PlayerResponse {
            id: player.id.map(|id| id.to_hex()).unwrap_or_default(),
            account_id: player.account_id,
            name: player.name,
            shard: player.shard,
            last_refreshed_at: player.last_refreshed_at,
            created_at: player.created_at,
            summary: player.summary,
        }
    }
}

impl Player {
    pub fn new(account_id: String, name: String, shard: String) -> Self {
        Player {
            id: None,
            account_id,
            name,
            shard,
            last_matches: Vec::new(),
            last_refreshed_at: None,
            created_at: Utc::now(),
            summary: None,
        }
    }
}
