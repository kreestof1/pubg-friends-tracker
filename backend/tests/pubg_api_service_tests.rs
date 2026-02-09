#[cfg(test)]
mod pubg_api_service_tests {
    use mockito::{Mock, Server};
    use pubg_tracker_api::services::PubgApiService;

    #[tokio::test]
    async fn test_get_player_by_name_success() {
        let mut server = Server::new_async().await;
        
        let mock_response = r#"{
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
                            {"type": "match", "id": "match2"}
                        ]
                    }
                }
            }]
        }"#;

        let _mock = server
            .mock("GET", "/steam/players?filter[playerNames]=TestPlayer")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response)
            .create_async()
            .await;

        let service = PubgApiService::new(
            "test-api-key".to_string(),
            server.url(),
        );

        let result = service.get_player_by_name("steam", "TestPlayer").await;
        
        assert!(result.is_ok());
        let player_response = result.unwrap();
        assert_eq!(player_response.data.len(), 1);
        assert_eq!(player_response.data[0].attributes.name, "TestPlayer");
        assert_eq!(player_response.data[0].relationships.matches.data.len(), 2);
    }

    #[tokio::test]
    async fn test_get_player_by_name_not_found() {
        let mut server = Server::new_async().await;
        
        let _mock = server
            .mock("GET", "/steam/players?filter[playerNames]=UnknownPlayer")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(r#"{"errors":[{"title":"Not Found"}]}"#)
            .create_async()
            .await;

        let service = PubgApiService::new(
            "test-api-key".to_string(),
            server.url(),
        );

        let result = service.get_player_by_name("steam", "UnknownPlayer").await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_player_by_name_rate_limit() {
        let mut server = Server::new_async().await;
        
        // First request: rate limited
        let mock1 = server
            .mock("GET", "/steam/players?filter[playerNames]=TestPlayer")
            .with_status(429)
            .with_header("X-RateLimit-Reset", "1")
            .with_body("Rate limit exceeded")
            .expect(1)
            .create_async()
            .await;

        // Second request: success
        let mock2 = server
            .mock("GET", "/steam/players?filter[playerNames]=TestPlayer")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"data":[{"type":"player","id":"account.test","attributes":{"name":"TestPlayer","shardId":"steam"},"relationships":{"matches":{"data":[]}}}]}"#)
            .expect(1)
            .create_async()
            .await;

        let service = PubgApiService::new(
            "test-api-key".to_string(),
            server.url(),
        );

        let result = service.get_player_by_name("steam", "TestPlayer").await;
        
        mock1.assert_async().await;
        mock2.assert_async().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_player_by_name_server_error_retry() {
        let mut server = Server::new_async().await;
        
        // First request: server error
        let mock1 = server
            .mock("GET", "/steam/players?filter[playerNames]=TestPlayer")
            .with_status(500)
            .with_body("Internal server error")
            .expect(1)
            .create_async()
            .await;

        // Second request: success
        let mock2 = server
            .mock("GET", "/steam/players?filter[playerNames]=TestPlayer")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"data":[{"type":"player","id":"account.test","attributes":{"name":"TestPlayer","shardId":"steam"},"relationships":{"matches":{"data":[]}}}]}"#)
            .expect(1)
            .create_async()
            .await;

        let service = PubgApiService::new(
            "test-api-key".to_string(),
            server.url(),
        );

        let result = service.get_player_by_name("steam", "TestPlayer").await;
        
        mock1.assert_async().await;
        mock2.assert_async().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_player_by_name_max_retries_exceeded() {
        let mut server = Server::new_async().await;
        
        // All requests fail
        let _mock = server
            .mock("GET", "/steam/players?filter[playerNames]=TestPlayer")
            .with_status(500)
            .with_body("Internal server error")
            .expect(4) // Initial + 3 retries
            .create_async()
            .await;

        let service = PubgApiService::new(
            "test-api-key".to_string(),
            server.url(),
        );

        let result = service.get_player_by_name("steam", "TestPlayer").await;
        
        assert!(result.is_err());
    }
}
