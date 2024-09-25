use anyhow::anyhow;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use itertools::Itertools as _;
use serde_json::Value as JsonValue;

pub type AppResult = std::result::Result<Json<JsonValue>, AppError>;

// Make our own error that wraps `anyhow::Error`.
pub struct AppError(anyhow::Error);

impl AppError {
    pub fn new(err: anyhow::Error) -> Self {
        AppError(err)
    }

    pub fn not_found(id: i64) -> Self {
        AppError::new(anyhow!("no todo with ID '{}' found", id))
    }
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        IntoResponse::into_response((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("error: {}", self.0.chain().join("; ")),
        ))
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
