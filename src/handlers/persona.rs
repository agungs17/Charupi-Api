use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use crate::{
    error::{AppError, AppResult},
    models::{CreatePersonaRequest, CreateUserPersonaRequest, Persona, UpdatePersonaRequest, UserPersona},
    state::AppState,
};

pub async fn list_personas(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    let rows = sqlx::query_as::<_, Persona>("SELECT * FROM personas ORDER BY updated_at DESC")
        .fetch_all(&state.db)
        .await?;

    Ok(Json(json!({ "success": true, "personas": rows })))
}

pub async fn get_persona(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<impl IntoResponse> {
    let row = sqlx::query_as::<_, Persona>("SELECT * FROM personas WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Persona {} not found", id)))?;

    Ok(Json(json!({ "success": true, "persona": row })))
}

pub async fn create_persona(
    State(state): State<AppState>,
    Json(payload): Json<CreatePersonaRequest>,
) -> AppResult<impl IntoResponse> {
    if payload.name.trim().is_empty() {
        return Err(AppError::Validation("Persona name cannot be empty".to_string()));
    }

    let now = chrono::Utc::now().timestamp_millis();
    let id = sqlx::query(
        r#"
        INSERT INTO personas (
            name, avatar_file, chat_bg_file, personality, appearance, relationship,
            story_background, first_message, extra_notes, message_examples, gender,
            created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.avatar_file)
    .bind(&payload.chat_bg_file)
    .bind(&payload.personality)
    .bind(&payload.appearance)
    .bind(&payload.relationship)
    .bind(&payload.story_background)
    .bind(&payload.first_message)
    .bind(&payload.extra_notes)
    .bind(&payload.message_examples)
    .bind(&payload.gender)
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await?
    .last_insert_rowid();

    // Jika first_message terisi, simpan pesan pembuka asisten
    if let Some(first_msg) = &payload.first_message {
        if !first_msg.trim().is_empty() {
            let _ = sqlx::query(
                "INSERT INTO messages (persona_id, role, content, created_at) VALUES (?, 'assistant', ?, ?)"
            )
            .bind(id)
            .bind(first_msg)
            .bind(now)
            .execute(&state.db)
            .await;
        }
    }

    let created = sqlx::query_as::<_, Persona>("SELECT * FROM personas WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await?;

    Ok(Json(json!({ "success": true, "persona": created })))
}

pub async fn update_persona(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePersonaRequest>,
) -> AppResult<impl IntoResponse> {
    let now = chrono::Utc::now().timestamp_millis();

    let mut existing = sqlx::query_as::<_, Persona>("SELECT * FROM personas WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Persona {} not found", id)))?;

    if let Some(name) = payload.name {
        existing.name = name;
    }
    if let Some(bg) = payload.story_background {
        existing.story_background = bg;
    }
    if let Some(avatar) = payload.avatar_file {
        existing.avatar_file = Some(avatar);
    }
    if let Some(chat_bg) = payload.chat_bg_file {
        existing.chat_bg_file = Some(chat_bg);
    }
    if let Some(p) = payload.personality {
        existing.personality = Some(p);
    }
    if let Some(app) = payload.appearance {
        existing.appearance = Some(app);
    }
    if let Some(rel) = payload.relationship {
        existing.relationship = Some(rel);
    }
    if let Some(notes) = payload.extra_notes {
        existing.extra_notes = Some(notes);
    }
    if let Some(ex) = payload.message_examples {
        existing.message_examples = Some(ex);
    }
    if let Some(g) = payload.gender {
        existing.gender = Some(g);
    }

    sqlx::query(
        r#"
        UPDATE personas SET
            name = ?, avatar_file = ?, chat_bg_file = ?, personality = ?, appearance = ?,
            relationship = ?, story_background = ?, extra_notes = ?, message_examples = ?,
            gender = ?, updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(&existing.name)
    .bind(&existing.avatar_file)
    .bind(&existing.chat_bg_file)
    .bind(&existing.personality)
    .bind(&existing.appearance)
    .bind(&existing.relationship)
    .bind(&existing.story_background)
    .bind(&existing.extra_notes)
    .bind(&existing.message_examples)
    .bind(&existing.gender)
    .bind(now)
    .bind(id)
    .execute(&state.db)
    .await?;

    Ok(Json(json!({ "success": true, "persona": existing })))
}

pub async fn delete_persona(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<impl IntoResponse> {
    sqlx::query("DELETE FROM personas WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;

    Ok(Json(json!({ "success": true, "message": "Persona deleted" })))
}

// User Persona Handlers
pub async fn list_user_personas(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    let rows = sqlx::query_as::<_, UserPersona>("SELECT * FROM user_persona ORDER BY id ASC")
        .fetch_all(&state.db)
        .await?;

    Ok(Json(json!({ "success": true, "user_personas": rows })))
}

pub async fn create_user_persona(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPersonaRequest>,
) -> AppResult<impl IntoResponse> {
    let now = chrono::Utc::now().timestamp_millis();
    let is_default = if payload.is_default.unwrap_or(false) { 1 } else { 0 };

    if is_default == 1 {
        let _ = sqlx::query("UPDATE user_persona SET is_default = 0").execute(&state.db).await;
    }

    let id = sqlx::query(
        r#"
        INSERT INTO user_persona (
            name, description, appearance, personality, gender, age, extra_notes, is_default, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(payload.appearance.unwrap_or_default())
    .bind(&payload.personality)
    .bind(&payload.gender)
    .bind(payload.age.unwrap_or_default())
    .bind(&payload.extra_notes)
    .bind(is_default)
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await?
    .last_insert_rowid();

    let created = sqlx::query_as::<_, UserPersona>("SELECT * FROM user_persona WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await?;

    Ok(Json(json!({ "success": true, "user_persona": created })))
}
