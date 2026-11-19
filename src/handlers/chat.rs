use std::convert::Infallible;
use axum::{
    extract::{Path, State},
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse, Response,
    },
    Json,
};
use futures::StreamExt;
use serde_json::json;
use crate::{
    error::{AppError, AppResult},
    models::{
        DbMessage, NextTurnRequest, Persona, RegenerateRequest, SendMessageRequest,
        SendMessageResponse, StoryCharacter, StoryEvent, UpstreamChatCompletionRequest,
        UserPersona,
    },
    services::{OmniRouteClient, PromptService, VectorService},
    state::AppState,
};

pub async fn list_messages(
    State(state): State<AppState>,
    Path(persona_id): Path<i64>,
) -> AppResult<impl IntoResponse> {
    let messages = sqlx::query_as::<_, DbMessage>(
        r#"
        SELECT id, persona_id, role, content, chunk_id, created_at
        FROM messages
        WHERE persona_id = ?
        ORDER BY id DESC
        LIMIT 50
        "#,
    )
    .bind(persona_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "success": true,
        "persona_id": persona_id,
        "messages": messages,
    })))
}

pub async fn send_message(
    State(state): State<AppState>,
    Json(payload): Json<SendMessageRequest>,
) -> AppResult<Response> {
    let now = chrono::Utc::now().timestamp_millis();

    // 1. Validasi persona
    let persona = sqlx::query_as::<_, Persona>("SELECT * FROM personas WHERE id = ?")
        .bind(payload.persona_id)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Persona {} not found", payload.persona_id)))?;

    // 2. Ambil User Persona
    let user = if let Some(uid) = payload.user_persona_id {
        sqlx::query_as::<_, UserPersona>("SELECT * FROM user_persona WHERE id = ?")
            .bind(uid)
            .fetch_optional(&state.db)
            .await?
    } else {
        sqlx::query_as::<_, UserPersona>("SELECT * FROM user_persona WHERE is_default = 1")
            .fetch_optional(&state.db)
            .await?
    }
    .unwrap_or(UserPersona {
        id: 0,
        name: "User".to_string(),
        description: None,
        appearance: String::new(),
        personality: None,
        gender: None,
        age: String::new(),
        extra_notes: None,
        is_default: 1,
        created_at: 0,
        updated_at: 0,
    });

    // 3. Simpan pesan pengguna ke database
    let user_msg_id = sqlx::query(
        "INSERT INTO messages (persona_id, role, content, created_at) VALUES (?, 'user', ?, ?)",
    )
    .bind(payload.persona_id)
    .bind(&payload.content)
    .bind(now)
    .execute(&state.db)
    .await?
    .last_insert_rowid();

    let user_db_message = DbMessage {
        id: user_msg_id,
        persona_id: payload.persona_id,
        role: "user".to_string(),
        content: payload.content.clone(),
        chunk_id: None,
        created_at: now,
    };

    // 4. Ambil karakter & event cerita yang aktif
    let characters = sqlx::query_as::<_, StoryCharacter>(
        "SELECT * FROM story_characters WHERE persona_id = ? AND is_active = 1",
    )
    .bind(payload.persona_id)
    .fetch_all(&state.db)
    .await?;

    let events = sqlx::query_as::<_, StoryEvent>(
        "SELECT * FROM story_events WHERE persona_id = ? AND is_active = 1",
    )
    .bind(payload.persona_id)
    .fetch_all(&state.db)
    .await?;

    // 5. Ambil 20 riwayat pesan terakhir
    let mut history = sqlx::query_as::<_, DbMessage>(
        r#"
        SELECT id, persona_id, role, content, chunk_id, created_at
        FROM messages
        WHERE persona_id = ?
        ORDER BY id DESC
        LIMIT 20
        "#,
    )
    .bind(payload.persona_id)
    .fetch_all(&state.db)
    .await?;
    history.reverse();

    // 6. Ambil memori RAG relevan jika ada
    let omni_client = OmniRouteClient::new(&state.config, state.http_client.clone());
    let mut memory_text = String::new();
    if let Ok(embeddings) = omni_client
        .create_embeddings(vec![payload.content.clone()], &state.config.omniroute_embedding_model)
        .await
    {
        if let Some(query_vec) = embeddings.first() {
            if let Ok(matches) = VectorService::search_relevant_memories(
                &state.db,
                payload.persona_id,
                query_vec,
                state.config.vector_top_k,
                state.config.vector_cosine_threshold,
            )
            .await
            {
                let snippets: Vec<String> = matches.into_iter().map(|m| format!("- {}", m.text)).collect();
                memory_text = snippets.join("\n");
            }
        }
    }

    // 7. Bangun System Prompt & Message Chain
    let messages = PromptService::build_prompt(
        &persona,
        &user,
        &characters,
        &events,
        if memory_text.is_empty() { None } else { Some(&memory_text) },
        &history,
    );

    let model = payload
        .model
        .unwrap_or_else(|| state.config.omniroute_default_model.clone());

    let upstream_req = UpstreamChatCompletionRequest {
        model,
        messages,
        temperature: 0.80,
        top_p: 0.90,
        min_p: Some(0.05),
        top_k: Some(40),
        repetition_penalty: Some(1.05),
        max_tokens: 1024,
        stream: payload.stream,
    };

    // 8. Handle Streaming vs Non-Streaming
    if payload.stream {
        let stream = omni_client.chat_completion_stream(&upstream_req).await?;
        let pool = state.db.clone();
        let persona_id = payload.persona_id;

        let sse_stream = async_stream::stream! {
            let mut byte_stream = stream;
            let mut full_accumulated = String::new();

            while let Some(chunk_res) = byte_stream.next().await {
                match chunk_res {
                    Ok(bytes) => {
                        if let Ok(text) = std::str::from_utf8(&bytes) {
                            // Kirimkan SSE chunk langsung ke client
                            yield Ok::<_, Infallible>(Event::default().data(text));

                            // Parser sederhana untuk mengumpulkan teks jawaban utuh
                            for line in text.lines() {
                                if let Some(data) = line.strip_prefix("data: ") {
                                    if data.trim() == "[DONE]" {
                                        continue;
                                    }
                                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(data) {
                                        if let Some(delta) = val["choices"][0]["delta"]["content"].as_str() {
                                            full_accumulated.push_str(delta);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => {
                        tracing::error!(error = ?err, "Error in upstream SSE chunk");
                        break;
                    }
                }
            }

            // Simpan hasil respons lengkap ke database
            if !full_accumulated.trim().is_empty() {
                let complete_now = chrono::Utc::now().timestamp_millis();
                let _ = sqlx::query(
                    "INSERT INTO messages (persona_id, role, content, created_at) VALUES (?, 'assistant', ?, ?)"
                )
                .bind(persona_id)
                .bind(&full_accumulated)
                .bind(complete_now)
                .execute(&pool)
                .await;
            }
        };

        Ok(Sse::new(sse_stream)
            .keep_alive(KeepAlive::default())
            .into_response())
    } else {
        let response = omni_client.chat_completion(&upstream_req).await?;
        let content = response
            .choices
            .first()
            .and_then(|c| c.message.as_ref())
            .map(|m| m.content.clone())
            .unwrap_or_default();

        let assistant_now = chrono::Utc::now().timestamp_millis();
        let assistant_id = sqlx::query(
            "INSERT INTO messages (persona_id, role, content, created_at) VALUES (?, 'assistant', ?, ?)"
        )
        .bind(payload.persona_id)
        .bind(&content)
        .bind(assistant_now)
        .execute(&state.db)
        .await?
        .last_insert_rowid();

        let assistant_db_message = DbMessage {
            id: assistant_id,
            persona_id: payload.persona_id,
            role: "assistant".to_string(),
            content,
            chunk_id: None,
            created_at: assistant_now,
        };

        Ok(Json(SendMessageResponse {
            message: user_db_message,
            assistant_reply: Some(assistant_db_message),
        })
        .into_response())
    }
}

pub async fn clear_messages(
    State(state): State<AppState>,
    Path(persona_id): Path<i64>,
) -> AppResult<impl IntoResponse> {
    sqlx::query("DELETE FROM messages WHERE persona_id = ?")
        .bind(persona_id)
        .execute(&state.db)
        .await?;

    Ok(Json(json!({
        "success": true,
        "message": "Chat history cleared successfully"
    })))
}
