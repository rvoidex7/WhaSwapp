use async_trait::async_trait;
use crate::types::*;
use super::AdapterResult;

/// Core trait that all messaging service adapters must implement
/// This allows the CLI to work with any messaging service
#[async_trait]
pub trait MessagingAdapter: Send + Sync {
    /// Get the name of this adapter (e.g., "WhatsApp", "Telegram", etc.)
    fn name(&self) -> &str;

    /// Connect to the messaging service
    async fn connect(&mut self) -> AdapterResult<()>;

    /// Disconnect from the messaging service
    async fn disconnect(&mut self) -> AdapterResult<()>;

    /// Get current connection status
    fn connection_status(&self) -> ConnectionStatus;

    /// Get list of all chats/conversations
    async fn get_chats(&self) -> AdapterResult<Vec<Chat>>;

    /// Get messages from a specific chat
    async fn get_messages(&self, chat_id: &ChatId, limit: usize) -> AdapterResult<Vec<Message>>;

    /// Send a text message to a chat
    async fn send_message(&mut self, chat_id: &ChatId, content: MessageContent) -> AdapterResult<Message>;

    /// Mark a message as read
    async fn mark_as_read(&mut self, chat_id: &ChatId, message_id: &MessageId) -> AdapterResult<()>;

    /// Get contact information
    async fn get_contact(&self, contact_id: &ContactId) -> AdapterResult<Contact>;

    /// Get list of all contacts
    async fn get_contacts(&self) -> AdapterResult<Vec<Contact>>;

    /// Listen for incoming messages (returns a receiver for new messages)
    async fn subscribe_to_messages(&mut self) -> AdapterResult<tokio::sync::mpsc::Receiver<Message>>;

    /// Search for messages or chats
    async fn search(&self, query: &str) -> AdapterResult<Vec<Chat>>;
}

/// Events that can be emitted by adapters
#[derive(Debug, Clone)]
pub enum AdapterEvent {
    MessageReceived(Message),
    MessageStatusChanged { message_id: MessageId, status: MessageStatus },
    ContactStatusChanged { contact_id: ContactId, is_online: bool },
    ConnectionStatusChanged(ConnectionStatus),
    Error(String),
}
