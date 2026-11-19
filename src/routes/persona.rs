use axum::{
    routing::{delete, get, post, put},
    Router,
};
use crate::{
    handlers::{
        create_persona, create_user_persona, delete_persona, get_persona, list_personas,
        list_user_personas, update_persona,
    },
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/personas", get(list_personas).post(create_persona))
        .route(
            "/personas/:id",
            get(get_persona).put(update_persona).delete(delete_persona),
        )
        .route(
            "/user-personas",
            get(list_user_personas).post(create_user_persona),
        )
}
