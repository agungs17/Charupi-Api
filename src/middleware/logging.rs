use axum::{body::Body, http::Request};
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::request_id::RequestId;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::Level;

pub fn trace_layer() -> TraceLayer<
    SharedClassifier<ServerErrorsAsFailures>,
    impl Fn(&Request<Body>) -> tracing::Span + Clone,
    tower_http::trace::DefaultOnRequest,
    DefaultOnResponse,
> {
    TraceLayer::new_for_http()
        .make_span_with(|request: &Request<Body>| {
            let request_id = request
                .extensions()
                .get::<RequestId>()
                .and_then(|id| id.header_value().to_str().ok())
                .unwrap_or("unknown");

            tracing::info_span!(
                "http_request",
                request_id = %request_id,
                method = %request.method(),
                uri = %request.uri(),
            )
        })
        .on_response(DefaultOnResponse::new().level(Level::INFO))
}
