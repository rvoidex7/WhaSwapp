# Creating Custom Messaging Adapters

This guide explains how to create a custom adapter for integrating your messaging service with cli-chat-rs.

## Overview

The cli-chat-rs framework uses a trait-based architecture that allows you to implement support for any messaging service by implementing the `MessagingAdapter` trait.

## Implementing a Custom Adapter

### 1. Add Dependencies

Create a new crate in the `adapters/` directory:

```bash
cd adapters
cargo new --lib your-adapter-name
```

Add cli-chat-rs as a dependency in your adapter's `Cargo.toml`:

```toml
[dependencies]
cli-chat-rs = { path = "../.." }
async-trait = "0.1"
tokio = { version = "1.35", features = ["full"] }
```

### 2. Implement the MessagingAdapter Trait

```rust
use async_trait::async_trait;
use cli_chat_rs::{
    MessagingAdapter, AdapterResult, Message, Chat, Contact,
    MessageContent, ConnectionStatus, ChatId, ContactId, MessageId,
};
use tokio::sync::mpsc;

pub struct YourAdapter {
    status: ConnectionStatus,
    // Add your service-specific fields
}

impl YourAdapter {
    pub fn new() -> Self {
        Self {
            status: ConnectionStatus::Disconnected,
        }
    }
}

#[async_trait]
impl MessagingAdapter for YourAdapter {
    fn name(&self) -> &str {
        "Your Service Name"
    }

    async fn connect(&mut self) -> AdapterResult<()> {
        // Implement connection logic to your messaging service
        self.status = ConnectionStatus::Connected;
        Ok(())
    }

    async fn disconnect(&mut self) -> AdapterResult<()> {
        // Implement disconnection logic
        self.status = ConnectionStatus::Disconnected;
        Ok(())
    }

    fn connection_status(&self) -> ConnectionStatus {
        self.status
    }

    async fn get_chats(&self) -> AdapterResult<Vec<Chat>> {
        // Fetch and return list of chats/conversations
        Ok(vec![])
    }

    async fn get_messages(&self, chat_id: &ChatId, limit: usize) -> AdapterResult<Vec<Message>> {
        // Fetch messages for a specific chat
        Ok(vec![])
    }

    async fn send_message(&mut self, chat_id: &ChatId, content: MessageContent) -> AdapterResult<Message> {
        // Send a message and return the sent message object
        todo!()
    }

    async fn mark_as_read(&mut self, chat_id: &ChatId, message_id: &MessageId) -> AdapterResult<()> {
        // Mark message as read
        Ok(())
    }

    async fn get_contact(&self, contact_id: &ContactId) -> AdapterResult<Contact> {
        // Get contact information
        todo!()
    }

    async fn get_contacts(&self) -> AdapterResult<Vec<Contact>> {
        // Get all contacts
        Ok(vec![])
    }

    async fn subscribe_to_messages(&mut self) -> AdapterResult<mpsc::Receiver<Message>> {
        let (tx, rx) = mpsc::channel(100);
        // Set up message listener that sends to tx
        Ok(rx)
    }

    async fn search(&self, query: &str) -> AdapterResult<Vec<Chat>> {
        // Implement search functionality
        Ok(vec![])
    }
}
```

### 3. Integration Methods

#### As a Submodule

Add your adapter as a git submodule:

```bash
git submodule add https://github.com/youruser/your-adapter adapters/your-adapter
```

#### As a Workspace Member

Add to the workspace in the root `Cargo.toml`:

```toml
[workspace]
members = ["adapters/your-adapter"]
```

#### As a Dependency

Add to dependencies in `Cargo.toml`:

```toml
[dependencies]
your-adapter = "0.1"
```

### 4. Using Your Adapter

```rust
use cli_chat_rs::{Config, MessengerApp};
use your_adapter::YourAdapter;

#[tokio::main]
async fn main() {
    let config = Config::default();
    let adapter = Box::new(YourAdapter::new());
    let mut app = MessengerApp::new(config, adapter);

    app.adapter_mut().connect().await.unwrap();
    // Use the app...
}
```

## Reference Implementations

See the following for examples:

- **Demo Adapter**: `src/adapter/demo.rs` (included in this repository)

## Configuration

Your adapter can read configuration from the `Config` object:

```rust
let adapter_config = config.adapters.get("your-adapter");
```

Add adapter-specific settings to the config file:

```json
{
  "active_adapter": "your-adapter",
  "adapters": {
    "your-adapter": {
      "enabled": true,
      "settings": {
        "api_key": "...",
        "server_url": "..."
      }
    }
  }
}
```

## Best Practices

1. **Error Handling**: Use `AdapterResult` for all async operations
2. **Async Operations**: Use tokio for async I/O
3. **Event Handling**: Use channels for real-time message updates
4. **State Management**: Keep connection state updated
5. **Testing**: Include unit tests for your adapter

## Support

For questions and issues, please open an issue on the cli-chat-rs repository.
