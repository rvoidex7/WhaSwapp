use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Message {
    pub id: String,
    pub chat_id: String,
    pub content: String,
    pub sender_id: String,
    pub timestamp: i64,
    pub from_me: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Chat {
    pub id: String,
    pub name: String,
    pub unread_count: u32,
    pub last_message_timestamp: i64,
}

#[async_trait]
pub trait Storage: Send + Sync {
    #[allow(dead_code)]
    async fn save_message(&self, message: Message) -> Result<(), Box<dyn Error + Send + Sync>>;
    #[allow(dead_code)]
    async fn get_messages(&self, chat_id: &str, limit: usize, offset: usize) -> Result<Vec<Message>, Box<dyn Error + Send + Sync>>;

    #[allow(dead_code)]
    async fn save_chat(&self, chat: Chat) -> Result<(), Box<dyn Error + Send + Sync>>;
    #[allow(dead_code)]
    async fn get_chats(&self) -> Result<Vec<Chat>, Box<dyn Error + Send + Sync>>;

    // Auth Data Management (for Baileys/Rust adapters)
    // Key is usually "auth_info" or specific keys like "creds"
    async fn save_auth_data(&self, key: &str, data: &serde_json::Value) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn get_auth_data(&self, key: &str) -> Result<Option<serde_json::Value>, Box<dyn Error + Send + Sync>>;
    #[allow(dead_code)]
    async fn remove_auth_data(&self, key: &str) -> Result<(), Box<dyn Error + Send + Sync>>;
}

pub mod db;
pub use db::SqliteStorage;
