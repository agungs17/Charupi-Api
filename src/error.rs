use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use crate::response::{ApiResponse, ValidationErrorDetail};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Token Expired")]
    TokenExpired,

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Conflict: {code} - {message}")]
    Conflict { message: String, code: String },

    #[error("Validation Error: {message}")]
    Validation {
        message: String,
        details: Vec<ValidationErrorDetail>,
    },

    #[error("Rate Limited")]
    RateLimited,

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Internal Server Error: {0}")]
    Internal(String),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message, code, details) = match self {
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                if msg.is_empty() { "Invalid request".to_string() } else { msg },
                "BAD_REQUEST".to_string(),
                None,
            ),
            AppError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                if msg.is_empty() { "Authentication required".to_string() } else { msg },
                "UNAUTHORIZED".to_string(),
                None,
            ),
            AppError::TokenExpired => (
                StatusCode::UNAUTHORIZED,
                "Session expired, please login again".to_string(),
                "TOKEN_EXPIRED".to_string(),
                None,
            ),
            AppError::Forbidden(msg) => (
                StatusCode::FORBIDDEN,
                if msg.is_empty() { "Access forbidden".to_string() } else { msg },
                "FORBIDDEN".to_string(),
                None,
            ),
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                if msg.is_empty() { "Resource not found".to_string() } else { msg },
                "NOT_FOUND".to_string(),
                None,
            ),
            AppError::Conflict { message, code } => (
                StatusCode::CONFLICT,
                message,
                code,
                None,
            ),
            AppError::Validation { message, details } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                if message.is_empty() { "Validation failed".to_string() } else { message },
                "VALIDATION_ERROR".to_string(),
                Some(details),
            ),
            AppError::RateLimited => (
                StatusCode::TOO_MANY_REQUESTS,
                "Too many requests, please try again later".to_string(),
                "RATE_LIMITED".to_string(),
                None,
            ),
            AppError::Serialization(err) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid data format: {}", err),
                "BAD_REQUEST".to_string(),
                None,
            ),
            AppError::Internal(msg) => {
                // Original details are logged to server, client response remains generic
                tracing::error!(error = %msg, "Internal server error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                    "INTERNAL_ERROR".to_string(),
                    None,
                )
            }
            AppError::Unexpected(err) => {
                // Internal error details logged to server
                tracing::error!(error = ?err, "Unexpected internal error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                    "INTERNAL_ERROR".to_string(),
                    None,
                )
            }
        };

        ApiResponse::error(status, message, code, details).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use serde_json::Value;

    async fn extract_response_body(response: Response) -> (StatusCode, Value) {
        let status = response.status();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let val: Value = serde_json::from_slice(&body).unwrap();
        (status, val)
    }

    #[tokio::test]
    async fn test_token_expired_response() {
        let err = AppError::TokenExpired;
        let (status, val) = extract_response_body(err.into_response()).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(val["success"], false);
        assert_eq!(val["message"], "Session expired, please login again");
        assert_eq!(val["code"], "TOKEN_EXPIRED");
        assert!(val.get("data").is_none());
    }

    #[tokio::test]
    async fn test_conflict_response() {
        let err = AppError::Conflict {
            message: "Email is already registered".to_string(),
            code: "EMAIL_TAKEN".to_string(),
        };
        let (status, val) = extract_response_body(err.into_response()).await;
        assert_eq!(status, StatusCode::CONFLICT);
        assert_eq!(val["success"], false);
        assert_eq!(val["message"], "Email is already registered");
        assert_eq!(val["code"], "EMAIL_TAKEN");
    }

    #[tokio::test]
    async fn test_internal_error_is_generic() {
        let err = AppError::Internal("Database connection lost: password incorrect".to_string());
        let (status, val) = extract_response_body(err.into_response()).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(val["success"], false);
        assert_eq!(val["message"], "Internal server error");
        assert_eq!(val["code"], "INTERNAL_ERROR");
        // Ensure sensitive internal credentials / SQL errors are not leaked
        assert_ne!(val["message"], "Database connection lost: password incorrect");
    }
}
