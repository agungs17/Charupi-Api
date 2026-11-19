use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Network / Upstream error: {0}")]
    Upstream(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Authentication or authorization failed: {0}")]
    Unauthorized(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Vector operation failed: {0}")]
    Vector(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone()),
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, "VALIDATION_ERROR", msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.clone()),
            AppError::Database(err) => {
                tracing::error!(error = ?err, "Database error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR",
                    "A database error occurred".to_string(),
                )
            }
            AppError::Upstream(err) => {
                tracing::error!(error = ?err, "Upstream OmniRoute gateway error");
                (
                    StatusCode::BAD_GATEWAY,
                    "UPSTREAM_GATEWAY_ERROR",
                    "Failed to communicate with OmniRoute LLM gateway".to_string(),
                )
            }
            AppError::Vector(msg) => {
                tracing::error!(error = ?msg, "Vector index error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "VECTOR_ERROR",
                    msg.clone(),
                )
            }
            AppError::Serialization(err) => (
                StatusCode::BAD_REQUEST,
                "SERIALIZATION_ERROR",
                err.to_string(),
            ),
            AppError::Internal(msg) => {
                tracing::error!(error = ?msg, "Internal application error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_SERVER_ERROR",
                    msg.clone(),
                )
            }
            AppError::Unexpected(err) => {
                tracing::error!(error = ?err, "Unexpected error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "UNEXPECTED_ERROR",
                    "An unexpected internal error occurred".to_string(),
                )
            }
        };

        let body = Json(ErrorResponse {
            success: false,
            error: message,
            code: error_code.to_string(),
            details: None,
        });

        (status, body).into_response()
    }
}
