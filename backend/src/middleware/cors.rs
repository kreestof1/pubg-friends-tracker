use tower_http::cors::{CorsLayer, Any};

pub fn create_cors_layer(allowed_origins: &str) -> CorsLayer {
    if allowed_origins == "*" {
        // Development mode - allow all
        CorsLayer::permissive()
    } else {
        // Production mode - restrict origins
        CorsLayer::new()
            .allow_origin(allowed_origins.parse::<axum::http::HeaderValue>().unwrap())
            .allow_methods(Any)
            .allow_headers(Any)
    }
}
