use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use crate::{
    error::{AppError, AppResult},
    models::{
        CreateStoryCharacterRequest, CreateStoryEventRequest, StoryCharacter, StoryEvent,
    },
    state::AppState,
};

pub async fn list_characters_by_persona(
    State(state): State<AppState>,
    Path(persona_id): Path<i64>,
) -> AppResult<impl IntoResponse> {
    let rows = sqlx::query_as::<_, StoryCharacter>(
        "SELECT * FROM story_characters WHERE persona_id = ? ORDER BY id ASC",
    )
    .bind(persona_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({ "success": true, "characters": rows })))
}

pub async fn create_character(
    State(state): State<AppState>,
    Json(payload): Json<CreateStoryCharacterRequest>,
) -> AppResult<impl IntoResponse> {
    if payload.name.trim().is_empty() {
        return Err(AppError::Validation("Character name cannot be empty".to_string()));
    }

    let now = chrono::Utc::now().timestamp_millis();
    let keywords_json = serde_json::to_string(&payload.keywords.unwrap_or_default())?;

    let id = sqlx::query(
        r#"
        INSERT INTO story_characters (
            persona_id, name, nickname, gender, age, appearance, personality, relationship,
            search_mode, keywords, is_active, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(payload.persona_id)
    .bind(&payload.name)
    .bind(payload.nickname.unwrap_or_default())
    .bind(&payload.gender)
    .bind(payload.age.unwrap_or_default())
    .bind(payload.appearance.unwrap_or_default())
    .bind(payload.personality.unwrap_or_default())
    .bind(payload.relationship.unwrap_or_default())
    .bind(payload.search_mode.unwrap_or_else(|| "pinned".to_string()))
    .bind(keywords_json)
    .bind(if payload.is_active.unwrap_or(true) { 1 } else { 0 })
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await?
    .last_insert_rowid();

    let created = sqlx::query_as::<_, StoryCharacter>("SELECT * FROM story_characters WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await?;

    Ok(Json(json!({ "success": true, "character": created })))
}

pub async fn delete_character(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<impl IntoResponse> {
    sqlx::query("DELETE FROM story_characters WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;

    Ok(Json(json!({ "success": true, "message": "Character deleted" })))
}

pub async fn list_events_by_persona(
    State(state): State<AppState>,
    Path(persona_id): Path<i64>,
) -> AppResult<impl IntoResponse> {
    let rows = sqlx::query_as::<_, StoryEvent>(
        "SELECT * FROM story_events WHERE persona_id = ? ORDER BY id ASC",
    )
    .bind(persona_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({ "success": true, "events": rows })))
}

pub async fn create_event(
    State(state): State<AppState>,
    Json(payload): Json<CreateStoryEventRequest>,
) -> AppResult<impl IntoResponse> {
    if payload.title.trim().is_empty() {
        return Err(AppError::Validation("Event title cannot be empty".to_string()));
    }

    let now = chrono::Utc::now().timestamp_millis();
    let keywords_json = serde_json::to_string(&payload.keywords.unwrap_or_default())?;

    let event_id = sqlx::query(
        r#"
        INSERT INTO story_events (
            persona_id, title, story, status, search_mode, keywords, is_active, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(payload.persona_id)
    .bind(&payload.title)
    .bind(&payload.story)
    .bind(payload.status.unwrap_or_else(|| "recent".to_string()))
    .bind(payload.search_mode.unwrap_or_else(|| "pinned".to_string()))
    .bind(keywords_json)
    .bind(if payload.is_active.unwrap_or(true) { 1 } else { 0 })
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await?
    .last_insert_rowid();

    // Kaitkan karakter yang terhubung
    if let Some(char_ids) = payload.character_ids {
        for cid in char_ids {
            let _ = sqlx::query(
                "INSERT OR IGNORE INTO story_event_characters (event_id, character_id) VALUES (?, ?)"
            )
            .bind(event_id)
            .bind(cid)
            .execute(&state.db)
            .await;
        }
    }

    let created = sqlx::query_as::<_, StoryEvent>("SELECT * FROM story_events WHERE id = ?")
        .bind(event_id)
        .fetch_one(&state.db)
        .await?;

    Ok(Json(json!({ "success": true, "event": created })))
}
