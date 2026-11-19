use axum::Router;
use tower_http::services::ServeDir;
use crate::state::AppState;

pub mod health;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(health::router())
        .nest_service("/docs", ServeDir::new("docs"))
        .with_state(state)
}
