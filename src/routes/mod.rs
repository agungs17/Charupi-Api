use axum::{response::IntoResponse, Router};
use tower_http::services::ServeDir;
use crate::{error::AppError, state::AppState};

pub mod health;

pub fn create_router(state: AppState) -> Router {
    let mut router = Router::new()
        .merge(health::router());

    // Dokumentasi API Scalar hanya diaktifkan pada environment development
    if state.config.is_development() {
        tracing::info!("Scalar API documentation enabled at /docs/");
        router = router.nest_service("/docs", ServeDir::new("docs"));
    }

    router
        .fallback(not_found_fallback)
        .with_state(state)
}

async fn not_found_fallback() -> impl IntoResponse {
    AppError::NotFound("Endpoint not found".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use serde_json::Value;
    use tower::ServiceExt;
    use crate::config::AppConfig;

    fn make_test_state(env: &str) -> AppState {
        let config = AppConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            environment: env.to_string(),
            database: crate::config::DatabaseConfig::default(),
        };
        AppState::new(config)
    }

    #[tokio::test]
    async fn test_docs_enabled_in_development() {
        let state = make_test_state("development");
        let app = create_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/docs/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // In development, /docs/ returns 200 OK
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_docs_disabled_in_production() {
        let state = make_test_state("production");
        let app = create_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/docs/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // In production, /docs/ is not registered and falls back to 404 Not Found
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["success"], false);
        assert_eq!(json["code"], "NOT_FOUND");
        assert_eq!(json["message"], "Endpoint not found");
    }

    #[tokio::test]
    async fn test_unknown_route_returns_standardized_404() {
        let state = make_test_state("development");
        let app = create_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/non-existent-endpoint")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["success"], false);
        assert_eq!(json["code"], "NOT_FOUND");
    }
}
