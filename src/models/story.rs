use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct StoryCharacter {
    pub id: i64,
    pub persona_id: i64,
    pub name: String,
    pub nickname: String,
    pub gender: Option<String>,
    pub age: String,
    pub appearance: String,
    pub personality: String,
    pub relationship: String,
    pub search_mode: String,
    pub keywords: String,
    pub is_active: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateStoryCharacterRequest {
    pub persona_id: i64,
    pub name: String,
    pub nickname: Option<String>,
    pub gender: Option<String>,
    pub age: Option<String>,
    pub appearance: Option<String>,
    pub personality: Option<String>,
    pub relationship: Option<String>,
    pub search_mode: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub is_active: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct StoryEvent {
    pub id: i64,
    pub persona_id: i64,
    pub title: String,
    pub story: String,
    pub status: String,
    pub search_mode: String,
    pub keywords: String,
    pub is_active: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateStoryEventRequest {
    pub persona_id: i64,
    pub title: String,
    pub story: String,
    pub status: Option<String>,
    pub search_mode: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub character_ids: Option<Vec<i64>>,
    pub is_active: Option<bool>,
}
