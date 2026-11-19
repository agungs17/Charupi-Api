use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Persona {
    pub id: i64,
    pub name: String,
    pub avatar_file: Option<String>,
    pub chat_bg_file: Option<String>,
    pub personality: Option<String>,
    pub appearance: Option<String>,
    pub relationship: Option<String>,
    pub story_background: String,
    pub first_message: Option<String>,
    pub extra_notes: Option<String>,
    pub message_examples: Option<String>,
    pub gender: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreatePersonaRequest {
    pub name: String,
    pub avatar_file: Option<String>,
    pub chat_bg_file: Option<String>,
    pub personality: Option<String>,
    pub appearance: Option<String>,
    pub relationship: Option<String>,
    pub story_background: String,
    pub first_message: Option<String>,
    pub extra_notes: Option<String>,
    pub message_examples: Option<String>,
    pub gender: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePersonaRequest {
    pub name: Option<String>,
    pub avatar_file: Option<String>,
    pub chat_bg_file: Option<String>,
    pub personality: Option<String>,
    pub appearance: Option<String>,
    pub relationship: Option<String>,
    pub story_background: Option<String>,
    pub first_message: Option<String>,
    pub extra_notes: Option<String>,
    pub message_examples: Option<String>,
    pub gender: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserPersona {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub appearance: String,
    pub personality: Option<String>,
    pub gender: Option<String>,
    pub age: String,
    pub extra_notes: Option<String>,
    pub is_default: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserPersonaRequest {
    pub name: String,
    pub description: Option<String>,
    pub appearance: Option<String>,
    pub personality: Option<String>,
    pub gender: Option<String>,
    pub age: Option<String>,
    pub extra_notes: Option<String>,
    pub is_default: Option<bool>,
}
