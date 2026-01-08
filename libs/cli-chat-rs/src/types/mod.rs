use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Unique identifier for messages
pub type MessageId = String;

/// Unique identifier for contacts/users
pub type ContactId = String;

/// Unique identifier for chats/conversations
pub type ChatId = String;

/// Represents a message in the messaging system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: MessageId,
    pub chat_id: ChatId,
    pub sender_id: ContactId,
    pub content: MessageContent,
    pub timestamp: DateTime<Utc>,
    pub is_from_me: bool,
    pub status: MessageStatus,
}

/// Different types of message content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageContent {
    Text(String),
    Image { path: String, caption: Option<String> },
    Video { path: String, caption: Option<String> },
    Audio { path: String, duration_secs: Option<u32> },
    Document { path: String, filename: String },
    Location { latitude: f64, longitude: f64, name: Option<String> },
}

/// Message delivery/read status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageStatus {
    Sending,
    Sent,
    Delivered,
    Read,
    Failed,
}

/// Represents a contact in the messaging system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: ContactId,
    pub name: String,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub status: Option<String>,
    pub is_online: bool,
}

/// Represents a chat/conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    pub id: ChatId,
    pub name: String,
    pub is_group: bool,
    pub participants: Vec<ContactId>,
    pub last_message: Option<Message>,
    pub unread_count: u32,
}

/// Connection status for the messaging service
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Failed,
}
