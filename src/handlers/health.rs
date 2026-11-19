use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;
use crate::state::AppState;

pub async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "service": "charupi-api",
        "version": env!("CARGO_PKG_VERSION"),
        "environment": state.config.environment,
    }))
}
