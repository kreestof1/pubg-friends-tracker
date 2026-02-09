use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub rust_env: String,
    pub host: String,
    pub port: u16,
    pub mongodb_uri: String,
    pub pubg_api_key: String,
    pub pubg_api_base_url: String,
    pub cors_origin: String,
    pub rust_log: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            rust_env: env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a valid u16"),
            mongodb_uri: env::var("MONGODB_URI")?,
            pubg_api_key: env::var("PUBG_API_KEY")?,
            pubg_api_base_url: env::var("PUBG_API_BASE_URL")
                .unwrap_or_else(|_| "https://api.pubg.com/shards".to_string()),
            cors_origin: env::var("CORS_ORIGIN").unwrap_or_else(|_| "*".to_string()),
            rust_log: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        })
    }

    pub fn is_production(&self) -> bool {
        self.rust_env == "production"
    }

    pub fn is_development(&self) -> bool {
        self.rust_env == "development"
    }
}
