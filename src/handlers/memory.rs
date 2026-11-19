use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use crate::{
    error::AppResult,
    models::{Chunk, SearchMemoryQuery},
    services::{OmniRouteClient, VectorService},
    state::AppState,
};

pub async fn search_memories(
    State(state): State<AppState>,
    Json(payload): Json<SearchMemoryQuery>,
) -> AppResult<impl IntoResponse> {
    let omni_client = OmniRouteClient::new(&state.config, state.http_client.clone());

    // 1. Dapatkan vektor embedding query dari OmniRoute
    let embeddings = omni_client
        .create_embeddings(vec![payload.query], &state.config.omniroute_embedding_model)
        .await?;

    let query_vector = embeddings
        .first()
        .cloned()
        .unwrap_or_default();

    let top_k = payload.limit.unwrap_or(state.config.vector_top_k);
    let threshold = payload.threshold.unwrap_or(state.config.vector_cosine_threshold);

    // 2. Cari kemiripan kosinus
    let matches = VectorService::search_relevant_memories(
        &state.db,
        payload.persona_id,
        &query_vector,
        top_k,
        threshold,
    )
    .await?;

    Ok(Json(json!({
        "success": true,
        "persona_id": payload.persona_id,
        "count": matches.len(),
        "results": matches,
    })))
}

pub async fn list_chunks_by_persona(
    State(state): State<AppState>,
    Path(persona_id): Path<i64>,
) -> AppResult<impl IntoResponse> {
    let rows = sqlx::query_as::<_, Chunk>(
        "SELECT id, persona_id, first_msg_id, last_msg_id, text, embedding, model, created_at FROM chunks WHERE persona_id = ? ORDER BY id DESC"
    )
    .bind(persona_id)
    .fetch_all(&state.db)
    .await?;

    let summaries: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|c| {
            json!({
                "id": c.id,
                "persona_id": c.persona_id,
                "first_msg_id": c.first_msg_id,
                "last_msg_id": c.last_msg_id,
                "text": c.text,
                "model": c.model,
                "created_at": c.created_at,
            })
        })
        .collect();

    Ok(Json(json!({ "success": true, "chunks": summaries })))
}

pub async fn clear_memories(
    State(state): State<AppState>,
    Path(persona_id): Path<i64>,
) -> AppResult<impl IntoResponse> {
    sqlx::query("DELETE FROM chunks WHERE persona_id = ?")
        .bind(persona_id)
        .execute(&state.db)
        .await?;

    Ok(Json(json!({
        "success": true,
        "message": "Persona memories cleared"
    })))
}
