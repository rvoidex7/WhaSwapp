use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod rust;
pub mod baileys;

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Message {
    pub jid: String,
    pub content: String,
    pub from_me: bool,
}

#[async_trait]
pub trait WhatsAppProvider: Send + Sync {
    /// Initialize the provider (e.g., spawn process, connect to WebSocket)
    async fn initialize(&self, payload: String) -> anyhow::Result<()>;

    /// Send a message
    async fn send_message(&self, jid: String, content: String) -> anyhow::Result<()>;

    /// Cleanup
    async fn disconnect(&self) -> anyhow::Result<()>;
}

// Re-export Manager for convenience
pub use crate::commands::WhatsAppManager;
