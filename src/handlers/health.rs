use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;
use crate::state::AppState;

pub async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    let db_ok = sqlx::query("SELECT 1;").execute(&state.db).await.is_ok();

    Json(json!({
        "status": if db_ok { "ok" } else { "degraded" },
        "service": "charupi-api",
        "version": env!("CARGO_PKG_VERSION"),
        "database": if db_ok { "connected" } else { "disconnected" },
        "omniroute_base_url": state.config.omniroute_base_url,
        "default_model": state.config.omniroute_default_model,
        "embedding_model": state.config.omniroute_embedding_model,
    }))
}
