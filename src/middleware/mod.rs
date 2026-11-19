pub mod cors;
pub mod logging;
pub mod panic;
pub mod request_id;

pub use cors::cors_layer;
pub use logging::trace_layer;
pub use panic::catch_panic_layer;
pub use request_id::{propagate_request_id_layer, set_request_id_layer, REQUEST_ID_HEADER};

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::extract::DefaultBodyLimit;
    use axum::http::{Request, StatusCode};
    use axum::routing::get;
    use axum::Router;
    use serde_json::Value;
    use tower::ServiceExt;

    async fn panic_handler() -> String {
        panic!("Simulated critical bug in handler");
    }

    fn build_test_app() -> Router {
        Router::new()
            .route(
                "/test/ok",
                get(|| async { axum::Json(serde_json::json!({"status": "ok"})) }),
            )
            .route("/test/panic", get(panic_handler))
            .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
            .layer(catch_panic_layer())
            .layer(cors_layer())
            .layer(trace_layer())
            .layer(propagate_request_id_layer())
            .layer(set_request_id_layer())
    }

    #[tokio::test]
    async fn test_request_id_generated_automatically() {
        let app = build_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/test/ok")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let header = response.headers().get(REQUEST_ID_HEADER);
        assert!(header.is_some(), "x-request-id header must exist");
        let val = header.unwrap().to_str().unwrap();
        assert!(!val.is_empty());
    }

    #[tokio::test]
    async fn test_request_id_propagated_from_client() {
        let app = build_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/test/ok")
                    .header(REQUEST_ID_HEADER, "custom-client-uuid-999")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let header = response.headers().get(REQUEST_ID_HEADER).unwrap();
        assert_eq!(header.to_str().unwrap(), "custom-client-uuid-999");
    }

    #[tokio::test]
    async fn test_panic_is_caught_and_returns_standard_500() {
        let app = build_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/test/panic")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Server does not crash, returns HTTP 500
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        // Header x-request-id is preserved even when panicking
        assert!(response.headers().get(REQUEST_ID_HEADER).is_some());

        // Body response follows standard Charupi format
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["success"], false);
        assert_eq!(json["code"], "INTERNAL_ERROR");
        assert_eq!(json["message"], "Internal server error");
    }
}
