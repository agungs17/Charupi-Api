use axum::Router;
use crate::state::AppState;

pub mod health;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(health::router())
        .with_state(state)
}
