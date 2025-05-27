use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProxyError {
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to convert body: {0}")]
    BodyConversionError(String),
}

impl IntoResponse for ProxyError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            ProxyError::BodyConversionError(e) => {
                (StatusCode::BAD_REQUEST, format!("Body error: {}", e))
            }
            ProxyError::RequestError(e) => {
                (StatusCode::BAD_GATEWAY, format!("Upstream error: {}", e))
            }
        };
        (status, msg).into_response()
    }
}
