use async_trait::async_trait;
use chrono::Utc;
use tokio::sync::mpsc;
use crate::adapter::{AdapterResult, MessagingAdapter};
use crate::types::*;

/// Demo adapter for testing and demonstration purposes
/// This shows how to implement a messaging adapter
pub struct DemoAdapter {
    status: ConnectionStatus,
    chats: Vec<Chat>,
    contacts: Vec<Contact>,
    messages: Vec<Message>,
}

impl DemoAdapter {
    pub fn new() -> Self {
        let contacts = vec![
            Contact {
                id: "contact1".to_string(),
                name: "Alice".to_string(),
                phone: Some("+1234567890".to_string()),
                avatar_url: None,
                status: Some("Hey there! I am using messenger".to_string()),
                is_online: true,
            },
            Contact {
                id: "contact2".to_string(),
                name: "Bob".to_string(),
                phone: Some("+0987654321".to_string()),
                avatar_url: None,
                status: Some("Busy".to_string()),
                is_online: false,
            },
        ];

        let chats = vec![
            Chat {
                id: "chat1".to_string(),
                name: "Alice".to_string(),
                is_group: false,
                participants: vec!["contact1".to_string()],
                last_message: None,
                unread_count: 0,
            },
            Chat {
                id: "chat2".to_string(),
                name: "Bob".to_string(),
                is_group: false,
                participants: vec!["contact2".to_string()],
                last_message: None,
                unread_count: 2,
            },
        ];

        Self {
            status: ConnectionStatus::Disconnected,
            chats,
            contacts,
            messages: Vec::new(),
        }
    }
}

#[async_trait]
impl MessagingAdapter for DemoAdapter {
    fn name(&self) -> &str {
        "Demo Adapter"
    }

    async fn connect(&mut self) -> AdapterResult<()> {
        self.status = ConnectionStatus::Connecting;
        // Simulate connection delay
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        self.status = ConnectionStatus::Connected;
        Ok(())
    }

    async fn disconnect(&mut self) -> AdapterResult<()> {
        self.status = ConnectionStatus::Disconnected;
        Ok(())
    }

    fn connection_status(&self) -> ConnectionStatus {
        self.status
    }

    async fn get_chats(&self) -> AdapterResult<Vec<Chat>> {
        Ok(self.chats.clone())
    }

    async fn get_messages(&self, chat_id: &ChatId, limit: usize) -> AdapterResult<Vec<Message>> {
        let messages: Vec<Message> = self.messages
            .iter()
            .filter(|m| &m.chat_id == chat_id)
            .take(limit)
            .cloned()
            .collect();
        Ok(messages)
    }

    async fn send_message(&mut self, chat_id: &ChatId, content: MessageContent) -> AdapterResult<Message> {
        let message = Message {
            id: format!("msg_{}", self.messages.len()),
            chat_id: chat_id.clone(),
            sender_id: "me".to_string(),
            content,
            timestamp: Utc::now(),
            is_from_me: true,
            status: MessageStatus::Sent,
        };
        self.messages.push(message.clone());
        Ok(message)
    }

    async fn mark_as_read(&mut self, _chat_id: &ChatId, _message_id: &MessageId) -> AdapterResult<()> {
        Ok(())
    }

    async fn get_contact(&self, contact_id: &ContactId) -> AdapterResult<Contact> {
        self.contacts
            .iter()
            .find(|c| &c.id == contact_id)
            .cloned()
            .ok_or_else(|| "Contact not found".into())
    }

    async fn get_contacts(&self) -> AdapterResult<Vec<Contact>> {
        Ok(self.contacts.clone())
    }

    async fn subscribe_to_messages(&mut self) -> AdapterResult<tokio::sync::mpsc::Receiver<Message>> {
        let (_tx, rx) = mpsc::channel(100);
        // In a real adapter, tx would be used to send incoming messages
        Ok(rx)
    }

    async fn search(&self, query: &str) -> AdapterResult<Vec<Chat>> {
        let query_lower = query.to_lowercase();
        let results: Vec<Chat> = self.chats
            .iter()
            .filter(|chat| chat.name.to_lowercase().contains(&query_lower))
            .cloned()
            .collect();
        Ok(results)
    }
}

impl Default for DemoAdapter {
    fn default() -> Self {
        Self::new()
    }
}
