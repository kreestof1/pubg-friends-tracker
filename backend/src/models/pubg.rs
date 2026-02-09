use serde::{Deserialize, Serialize};

// PUBG API Player Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubgPlayerResponse {
    pub data: Vec<PubgPlayerData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubgPlayerData {
    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub attributes: PubgPlayerAttributes,
    pub relationships: PubgPlayerRelationships,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubgPlayerAttributes {
    pub name: String,
    #[serde(rename = "shardId")]
    pub shard_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubgPlayerRelationships {
    pub matches: PubgMatchesData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubgMatchesData {
    pub data: Vec<PubgMatchRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubgMatchRef {
    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
}

// PUBG API Match Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubgMatchResponse {
    pub data: PubgMatchData,
    pub included: Vec<PubgMatchIncluded>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubgMatchData {
    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub attributes: PubgMatchAttributes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubgMatchAttributes {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub duration: i32,
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "mapName")]
    pub map_name: String,
    #[serde(rename = "isCustomMatch")]
    pub is_custom_match: bool,
    #[serde(rename = "matchType")]
    pub match_type: Option<String>,
    #[serde(rename = "shardId")]
    pub shard_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PubgMatchIncluded {
    #[serde(rename = "participant")]
    Participant {
        id: String,
        attributes: PubgParticipantAttributes,
    },
    #[serde(rename = "roster")]
    Roster {
        id: String,
        attributes: PubgRosterAttributes,
    },
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubgParticipantAttributes {
    pub stats: PubgParticipantStats,
    pub actor: Option<String>,
    #[serde(rename = "shardId")]
    pub shard_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubgParticipantStats {
    #[serde(rename = "DBNOs")]
    pub dbnos: i32,
    pub assists: i32,
    pub boosts: i32,
    #[serde(rename = "damageDealt")]
    pub damage_dealt: f64,
    #[serde(rename = "deathType")]
    pub death_type: String,
    #[serde(rename = "headshotKills")]
    pub headshot_kills: i32,
    pub heals: i32,
    #[serde(rename = "killPlace")]
    pub kill_place: i32,
    #[serde(rename = "killStreaks")]
    pub kill_streaks: i32,
    pub kills: i32,
    #[serde(rename = "longestKill")]
    pub longest_kill: f64,
    pub name: String,
    #[serde(rename = "playerId")]
    pub player_id: String,
    pub revives: i32,
    #[serde(rename = "rideDistance")]
    pub ride_distance: f64,
    #[serde(rename = "roadKills")]
    pub road_kills: i32,
    #[serde(rename = "swimDistance")]
    pub swim_distance: f64,
    #[serde(rename = "teamKills")]
    pub team_kills: i32,
    #[serde(rename = "timeSurvived")]
    pub time_survived: f64,
    #[serde(rename = "vehicleDestroys")]
    pub vehicle_destroys: i32,
    #[serde(rename = "walkDistance")]
    pub walk_distance: f64,
    #[serde(rename = "weaponsAcquired")]
    pub weapons_acquired: i32,
    #[serde(rename = "winPlace")]
    pub win_place: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubgRosterAttributes {
    pub stats: PubgRosterStats,
    #[serde(rename = "shardId")]
    pub shard_id: String,
    pub won: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubgRosterStats {
    pub rank: i32,
    #[serde(rename = "teamId")]
    pub team_id: i32,
}

// Error Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubgErrorResponse {
    pub errors: Vec<PubgError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubgError {
    pub title: String,
    pub detail: String,
}
