use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub player_id: ObjectId,
    pub period: String,      // "7d", "30d", "90d"
    pub mode: String,        // "solo", "duo", "squad", "all"
    pub shard: String,       // "steam", "xbox", "psn"
    pub kills: i32,
    pub deaths: i32,
    pub kd_ratio: f64,
    pub win_rate: f64,
    pub damage_dealt: f64,
    pub survival_time: f64,  // en secondes
    pub top1_count: i32,
    pub matches_played: i32,
    pub computed_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsFilter {
    pub period: Option<String>,
    pub mode: Option<String>,
    pub shard: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StatsResponse {
    pub player_id: String,
    pub period: String,
    pub mode: String,
    pub shard: String,
    pub kills: i32,
    pub deaths: i32,
    pub kd_ratio: f64,
    pub win_rate: f64,
    pub damage_dealt: f64,
    pub survival_time: f64,
    pub top1_count: i32,
    pub matches_played: i32,
    pub computed_at: DateTime<Utc>,
}

impl From<PlayerStats> for StatsResponse {
    fn from(stats: PlayerStats) -> Self {
        StatsResponse {
            player_id: stats.player_id.to_hex(),
            period: stats.period,
            mode: stats.mode,
            shard: stats.shard,
            kills: stats.kills,
            deaths: stats.deaths,
            kd_ratio: stats.kd_ratio,
            win_rate: stats.win_rate,
            damage_dealt: stats.damage_dealt,
            survival_time: stats.survival_time,
            top1_count: stats.top1_count,
            matches_played: stats.matches_played,
            computed_at: stats.computed_at,
        }
    }
}

impl PlayerStats {
    pub fn new(
        player_id: ObjectId,
        period: String,
        mode: String,
        shard: String,
        ttl_hours: i64,
    ) -> Self {
        let now = Utc::now();
        PlayerStats {
            id: None,
            player_id,
            period,
            mode,
            shard,
            kills: 0,
            deaths: 0,
            kd_ratio: 0.0,
            win_rate: 0.0,
            damage_dealt: 0.0,
            survival_time: 0.0,
            top1_count: 0,
            matches_played: 0,
            computed_at: now,
            expires_at: now + chrono::Duration::hours(ttl_hours),
        }
    }
}
