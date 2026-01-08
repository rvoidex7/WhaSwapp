// Example: Simple custom adapter implementation
use async_trait::async_trait;
use cli_chat_rs::{
    MessagingAdapter, AdapterResult, Message, Chat, Contact,
    MessageContent, ConnectionStatus, ChatId, ContactId, MessageId,
};
use tokio::sync::mpsc;

/// Example custom adapter
pub struct CustomAdapter {
    status: ConnectionStatus,
}

impl CustomAdapter {
    pub fn new() -> Self {
        Self {
            status: ConnectionStatus::Disconnected,
        }
    }
}

#[async_trait]
impl MessagingAdapter for CustomAdapter {
    fn name(&self) -> &str {
        "Custom Adapter Example"
    }

    async fn connect(&mut self) -> AdapterResult<()> {
        println!("Connecting to custom service...");
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
        Ok(vec![])
    }

    async fn get_messages(&self, _chat_id: &ChatId, _limit: usize) -> AdapterResult<Vec<Message>> {
        Ok(vec![])
    }

    async fn send_message(&mut self, chat_id: &ChatId, content: MessageContent) -> AdapterResult<Message> {
        println!("Sending message to {}: {:?}", chat_id, content);
        Err("Not implemented".into())
    }

    async fn mark_as_read(&mut self, _chat_id: &ChatId, _message_id: &MessageId) -> AdapterResult<()> {
        Ok(())
    }

    async fn get_contact(&self, _contact_id: &ContactId) -> AdapterResult<Contact> {
        Err("Not implemented".into())
    }

    async fn get_contacts(&self) -> AdapterResult<Vec<Contact>> {
        Ok(vec![])
    }

    async fn subscribe_to_messages(&mut self) -> AdapterResult<mpsc::Receiver<Message>> {
        let (_tx, rx) = mpsc::channel(100);
        Ok(rx)
    }

    async fn search(&self, _query: &str) -> AdapterResult<Vec<Chat>> {
        Ok(vec![])
    }
}

#[tokio::main]
async fn main() {
    use cli_chat_rs::{Config, MessengerApp};

    let config = Config::default();
    let adapter = Box::new(CustomAdapter::new());
    let mut app = MessengerApp::new(config, adapter);

    println!("Using adapter: {}", app.adapter().name());

    if let Err(e) = app.adapter_mut().connect().await {
        eprintln!("Connection error: {}", e);
        return;
    }
    println!("Connected!");

    match app.adapter().get_chats().await {
        Ok(chats) => println!("Chats: {:?}", chats),
        Err(e) => eprintln!("Error getting chats: {}", e),
    }

    if let Err(e) = app.adapter_mut().disconnect().await {
        eprintln!("Disconnect error: {}", e);
        return;
    }
    println!("Disconnected!");
}
