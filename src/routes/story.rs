use axum::{
    routing::{delete, get, post},
    Router,
};
use crate::{
    handlers::{
        create_character, create_event, delete_character, list_characters_by_persona,
        list_events_by_persona,
    },
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/story/characters/:persona_id",
            get(list_characters_by_persona),
        )
        .route("/story/characters", post(create_character))
        .route("/story/characters/:id", delete(delete_character))
        .route("/story/events/:persona_id", get(list_events_by_persona))
        .route("/story/events", post(create_event))
}
