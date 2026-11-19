use axum::http::HeaderName;
use tower_http::cors::{Any, CorsLayer};
use crate::middleware::request_id::REQUEST_ID_HEADER;

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .expose_headers([HeaderName::from_static(REQUEST_ID_HEADER)])
}
