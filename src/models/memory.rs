use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Chunk {
    pub id: i64,
    pub persona_id: i64,
    pub first_msg_id: i64,
    pub last_msg_id: i64,
    pub text: String,
    pub embedding: Vec<u8>,
    pub model: String,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchMemoryQuery {
    pub persona_id: i64,
    pub query: String,
    pub limit: Option<usize>,
    pub threshold: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemorySearchResult {
    pub id: i64,
    pub text: String,
    pub distance: f32,
    pub first_msg_id: i64,
    pub last_msg_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingRequest {
    pub input: Vec<String>,
    pub model: String,
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingDataItem {
    pub embedding: Vec<f32>,
    pub index: usize,
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingResponse {
    pub data: Vec<EmbeddingDataItem>,
    pub model: String,
}
