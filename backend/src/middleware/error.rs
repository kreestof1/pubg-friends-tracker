use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub async fn handle_errors(request: Request, next: Next) -> Response {
    let response = next.run(request).await;

    // Log errors based on status code
    let status = response.status();
    
    if status.is_server_error() {
        tracing::error!(
            status = %status,
            "Server error occurred"
        );
    } else if status.is_client_error() {
        tracing::warn!(
            status = %status,
            "Client error occurred"
        );
    }

    response
}
