use axum::response::IntoResponse;
use crate::response::ApiResponse;

pub async fn health_check() -> impl IntoResponse {
    ApiResponse::ok_msg("Service is healthy")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use axum::http::StatusCode;
    use serde_json::Value;

    #[tokio::test]
    async fn test_health_check_response_has_no_data() {
        let response = health_check().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["success"], true);
        assert_eq!(json["message"], "Service is healthy");
        assert!(json.get("data").is_none());
        assert!(json.get("meta").is_none());
        assert!(json.get("code").is_none());
    }
}
