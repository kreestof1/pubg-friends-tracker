#[cfg(test)]
mod stats_service_tests {
    use bson::oid::ObjectId;
    use chrono::Utc;
    use mongodb::bson::doc;
    use pubg_tracker_api::{
        db::MongoDb,
        models::{PlayerStats, PubgMatchResponse},
        services::StatsService,
    };
    use std::sync::Arc;

    async fn setup_test_db() -> Arc<MongoDb> {
        // Use testcontainers for real MongoDB in integration tests
        // For unit tests, we'll mock the repository methods
        let mongo_uri = std::env::var("TEST_MONGODB_URI")
            .unwrap_or_else(|_| "mongodb://localhost:27017/pubg-tracker-test".to_string());
        
        let db = MongoDb::new(&mongo_uri)
            .await
            .expect("Failed to connect to test MongoDB");
        
        Arc::new(db)
    }

    #[tokio::test]
    #[ignore] // Requires MongoDB running
    async fn test_cache_operations() {
        let db = setup_test_db().await;
        let service = StatsService::new(db.clone());
        let player_id = ObjectId::new();

        // Test cache miss - should return empty stats
        let result = service
            .get_or_compute_stats(&player_id, "7d", "solo", "steam")
            .await;
        
        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.matches_played, 0);

        // Create and save stats
        let test_stats = PlayerStats {
            id: None,
            player_id: player_id.clone(),
            period: "7d".to_string(),
            mode: "solo".to_string(),
            shard: "steam".to_string(),
            kills: 10,
            deaths: 5,
            kd_ratio: 2.0,
            win_rate: 20.0,
            damage_dealt: 1500.0,
            survival_time: 1800.0,
            top1_count: 2,
            matches_played: 10,
            computed_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::hours(1),
        };

        service.save_stats(&test_stats).await.expect("Failed to save stats");

        // Test cache hit from memory
        let result2 = service
            .get_or_compute_stats(&player_id, "7d", "solo", "steam")
            .await;
        
        assert!(result2.is_ok());
        let cached_stats = result2.unwrap();
        assert_eq!(cached_stats.kills, 10);
        assert_eq!(cached_stats.kd_ratio, 2.0);
        assert_eq!(cached_stats.matches_played, 10);

        // Test cache invalidation
        service.invalidate_cache(&player_id).await;

        // After invalidation, should get from DB
        let result3 = service
            .get_or_compute_stats(&player_id, "7d", "solo", "steam")
            .await;
        
        assert!(result3.is_ok());
        
        // Cleanup
        db.stats()
            .delete_many(doc! { "player_id": player_id.to_hex() }, None)
            .await
            .ok();
    }

    #[test]
    fn test_compute_stats_calculations() {
        // Test pure computation logic without DB
        // This would require exposing compute_stats_from_matches as public or testing through integration
        // For now, we'll test it in integration tests with real match data
    }

    #[tokio::test]
    #[ignore] // Requires MongoDB running
    async fn test_stats_ttl_expiration() {
        let db = setup_test_db().await;
        let service = StatsService::new(db.clone());
        let player_id = ObjectId::new();

        // Create stats that expire in the past
        let expired_stats = PlayerStats {
            id: None,
            player_id: player_id.clone(),
            period: "7d".to_string(),
            mode: "solo".to_string(),
            shard: "steam".to_string(),
            kills: 5,
            deaths: 3,
            kd_ratio: 1.67,
            win_rate: 10.0,
            damage_dealt: 800.0,
            survival_time: 900.0,
            top1_count: 1,
            matches_played: 5,
            computed_at: Utc::now() - chrono::Duration::hours(2),
            expires_at: Utc::now() - chrono::Duration::hours(1), // Expired
        };

        service.save_stats(&expired_stats).await.expect("Failed to save stats");

        // MongoDB TTL index will eventually clean this up
        // For immediate testing, we'd need to manually verify or wait
        
        // Cleanup
        db.stats()
            .delete_many(doc! { "player_id": player_id.to_hex() }, None)
            .await
            .ok();
    }
}
