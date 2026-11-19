use std::any::Any;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tower_http::catch_panic::CatchPanicLayer;
use crate::response::ApiResponse;

pub fn catch_panic_layer() -> CatchPanicLayer<fn(Box<dyn Any + Send + 'static>) -> Response> {
    CatchPanicLayer::custom(handle_panic)
}

fn handle_panic(err: Box<dyn Any + Send + 'static>) -> Response {
    let details = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Unknown panic payload".to_string()
    };

    tracing::error!(
        panic_details = %details,
        "Server caught unhandled panic in handler - recovered gracefully"
    );

    // Return standard Charupi error response format
    ApiResponse::error(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal server error",
        "INTERNAL_ERROR",
        None,
    )
    .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[tokio::test]
    async fn test_panic_handler_returns_standard_error() {
        let boxed_panic: Box<dyn Any + Send + 'static> = Box::new("Simulated critical panic");
        let response = handle_panic(boxed_panic);

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["success"], false);
        assert_eq!(json["code"], "INTERNAL_ERROR");
        assert_eq!(json["message"], "Internal server error");
        assert!(json.get("data").is_none());
    }
}
