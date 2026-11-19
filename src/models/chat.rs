use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbMessage {
    pub id: i64,
    pub persona_id: i64,
    pub role: String,
    pub content: String,
    pub chunk_id: Option<i64>,
    pub created_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub persona_id: i64,
    pub content: String,
    pub user_persona_id: Option<i64>,
    #[serde(default = "default_stream")]
    pub stream: bool,
    pub model: Option<String>,
}

fn default_stream() -> bool {
    true
}

#[derive(Debug, Serialize)]
pub struct SendMessageResponse {
    pub message: DbMessage,
    pub assistant_reply: Option<DbMessage>,
}

#[derive(Debug, Deserialize)]
pub struct NextTurnRequest {
    pub persona_id: i64,
    pub user_persona_id: Option<i64>,
    #[serde(default = "default_stream")]
    pub stream: bool,
    pub model: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RegenerateRequest {
    pub persona_id: i64,
    pub user_persona_id: Option<i64>,
    #[serde(default = "default_stream")]
    pub stream: bool,
    pub model: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpstreamChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
    pub top_p: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repetition_penalty: Option<f32>,
    pub max_tokens: u32,
    pub stream: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpstreamChatChoice {
    pub message: Option<ChatMessage>,
    pub delta: Option<ChatMessage>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpstreamChatCompletionResponse {
    pub id: Option<String>,
    pub choices: Vec<UpstreamChatChoice>,
}
