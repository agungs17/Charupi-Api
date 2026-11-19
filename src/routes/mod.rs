use axum::Router;
use crate::state::AppState;

pub mod chat;
pub mod health;
pub mod memory;
pub mod persona;
pub mod story;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(health::router())
        .nest("/api/v1", persona::router())
        .nest("/api/v1", story::router())
        .nest("/api/v1", chat::router())
        .nest("/api/v1", memory::router())
        .with_state(state)
}
