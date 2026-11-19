use axum::{
    routing::{delete, get, post},
    Router,
};
use crate::{
    handlers::{clear_memories, list_chunks_by_persona, search_memories},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/memory/search", post(search_memories))
        .route("/memory/chunks/:persona_id", get(list_chunks_by_persona))
        .route("/memory/clear/:persona_id", delete(clear_memories))
}
