use reqwest::{Client, header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION}};
use std::time::Duration;
use tokio::time::sleep;

use crate::models::{PubgPlayerResponse, PubgMatchResponse};

#[derive(Debug)]
pub enum PubgApiError {
    NotFound(String),
    RateLimit { retry_after: u64 },
    Unauthorized,
    ServerError(String),
    NetworkError(String),
}

impl std::fmt::Display for PubgApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PubgApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            PubgApiError::RateLimit { retry_after } => {
                write!(f, "Rate limit exceeded, retry after {} seconds", retry_after)
            }
            PubgApiError::Unauthorized => write!(f, "Unauthorized: Invalid API key"),
            PubgApiError::ServerError(msg) => write!(f, "Server error: {}", msg),
            PubgApiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for PubgApiError {}

pub struct PubgApiService {
    client: Client,
    api_key: String,
    base_url: String,
}

impl PubgApiService {
    pub fn new(api_key: String, base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        PubgApiService {
            client,
            api_key,
            base_url,
        }
    }

    fn create_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key))
                .expect("Invalid API key format"),
        );
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.api+json"),
        );
        headers
    }

    #[tracing::instrument(skip(self), fields(shard = %shard, player_name = %player_name))]
    pub async fn get_player_by_name(
        &self,
        shard: &str,
        player_name: &str,
    ) -> Result<PubgPlayerResponse, PubgApiError> {
        tracing::debug!("Requesting player data from PUBG API");
        let url = format!(
            "{}/{}/players?filter[playerNames]={}",
            self.base_url, shard, player_name
        );

        self.make_request_with_retry(&url, 3).await
    }

    pub async fn get_match(
        &self,
        shard: &str,
        match_id: &str,
    ) -> Result<PubgMatchResponse, PubgApiError> {
        let url = format!("{}/{}/matches/{}", self.base_url, shard, match_id);

        self.make_request_with_retry(&url, 3).await
    }

    async fn make_request_with_retry<T>(
        &self,
        url: &str,
        max_retries: u32,
    ) -> Result<T, PubgApiError>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut retries = 0;
        let mut backoff = Duration::from_secs(1);

        loop {
            let response = match self.client.get(url).headers(self.create_headers()).send().await {
                Ok(resp) => resp,
                Err(e) => {
                    if retries >= max_retries {
                        return Err(PubgApiError::NetworkError(e.to_string()));
                    }
                    tracing::warn!("Network error, retrying in {:?}: {}", backoff, e);
                    sleep(backoff).await;
                    retries += 1;
                    backoff *= 2;
                    continue;
                }
            };

            let status = response.status();

            // Check rate limit headers
            if let Some(rate_limit_reset) = response.headers().get("X-RateLimit-Reset") {
                if let Ok(reset_str) = rate_limit_reset.to_str() {
                    tracing::debug!("Rate limit will reset at: {}", reset_str);
                }
            }

            match status.as_u16() {
                200 => {
                    tracing::info!("Successfully fetched data from PUBG API");
                    return response.json::<T>().await.map_err(|e| {
                        PubgApiError::ServerError(format!("Failed to parse response: {}", e))
                    });
                }
                404 => {
                    let error_body = response.text().await.unwrap_or_default();
                    return Err(PubgApiError::NotFound(error_body));
                }
                401 | 403 => {
                    return Err(PubgApiError::Unauthorized);
                }
                429 => {
                    // Rate limit exceeded
                    let retry_after = response
                        .headers()
                        .get("X-RateLimit-Reset")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|s| s.parse::<u64>().ok())
                        .unwrap_or(60);

                    if retries >= max_retries {
                        return Err(PubgApiError::RateLimit { retry_after });
                    }

                    tracing::warn!(
                        "Rate limit exceeded, waiting {} seconds before retry",
                        retry_after
                    );
                    sleep(Duration::from_secs(retry_after)).await;
                    retries += 1;
                    continue;
                }
                500..=599 => {
                    // Server error, retry with backoff
                    if retries >= max_retries {
                        let error_body = response.text().await.unwrap_or_default();
                        return Err(PubgApiError::ServerError(format!(
                            "Status {}: {}",
                            status, error_body
                        )));
                    }

                    tracing::warn!(
                        "Server error {}, retrying in {:?}",
                        status,
                        backoff
                    );
                    sleep(backoff).await;
                    retries += 1;
                    backoff *= 2;
                    continue;
                }
                _ => {
                    let error_body = response.text().await.unwrap_or_default();
                    return Err(PubgApiError::ServerError(format!(
                        "Unexpected status {}: {}",
                        status, error_body
                    )));
                }
            }
        }
    }
}
