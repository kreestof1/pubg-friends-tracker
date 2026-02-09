// Common test utilities
use std::sync::Arc;
use pubg_tracker_api::db::MongoDb;

pub async fn setup_test_mongodb() -> Arc<MongoDb> {
    let mongo_uri = std::env::var("TEST_MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017/pubg-tracker-test".to_string());
    
    let db = MongoDb::new(&mongo_uri)
        .await
        .expect("Failed to connect to test MongoDB");
    
    Arc::new(db)
}

pub async fn cleanup_test_data(db: &MongoDb) {
    // Clean up test data after tests
    db.players().drop(None).await.ok();
    db.stats().drop(None).await.ok();
    db.create_indexes().await.ok();
}

#[cfg(test)]
pub fn mock_pubg_player_response() -> &'static str {
    r#"{
        "data": [{
            "type": "player",
            "id": "account.test123",
            "attributes": {
                "name": "TestPlayer",
                "shardId": "steam"
            },
            "relationships": {
                "matches": {
                    "data": [
                        {"type": "match", "id": "match1"},
                        {"type": "match", "id": "match2"},
                        {"type": "match", "id": "match3"}
                    ]
                }
            }
        }]
    }"#
}

#[cfg(test)]
pub fn mock_pubg_match_response() -> &'static str {
    r#"{
        "data": {
            "type": "match",
            "id": "match1",
            "attributes": {
                "gameMode": "solo",
                "duration": 1800,
                "createdAt": "2024-01-15T10:00:00Z"
            },
            "relationships": {
                "rosters": {
                    "data": [
                        {"type": "roster", "id": "roster1"}
                    ]
                }
            }
        },
        "included": [
            {
                "type": "participant",
                "id": "participant1",
                "attributes": {
                    "stats": {
                        "kills": 5,
                        "assists": 2,
                        "damageDealt": 750.5,
                        "timeSurvived": 1500,
                        "winPlace": 1,
                        "longestKill": 150.0,
                        "headshotKills": 2,
                        "revives": 0,
                        "teamKills": 0,
                        "vehicleDestroys": 0,
                        "roadKills": 0,
                        "killStreaks": 2,
                        "weaponsAcquired": 5,
                        "boosts": 3,
                        "heals": 5,
                        "DBNOs": 0,
                        "walkDistance": 2500.0,
                        "rideDistance": 1000.0,
                        "swimDistance": 0.0
                    }
                }
            }
        ]
    }"#
}
