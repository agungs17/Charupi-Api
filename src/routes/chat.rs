use axum::{
    routing::{delete, get, post},
    Router,
};
use crate::{
    handlers::{clear_messages, list_messages, send_message},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/chat/messages/:persona_id", get(list_messages))
        .route("/chat/send", post(send_message))
        .route("/chat/clear/:persona_id", delete(clear_messages))
}
