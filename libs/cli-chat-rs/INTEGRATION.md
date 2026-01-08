# Integration Examples

This document provides step-by-step instructions for integrating various messaging services with cli-chat-rs.

## Table of Contents

- [Custom Adapter](#custom-adapter)

## Custom Adapter

### Step 1: Create Adapter Crate

```bash
mkdir -p adapters/my-messenger
cd adapters/my-messenger
cargo init --lib
```

### Step 2: Add Dependencies

In `adapters/my-messenger/Cargo.toml`:

```toml
[package]
name = "my-messenger-adapter"
version = "0.1.0"
edition = "2021"

[dependencies]
cli-chat-rs = { path = "../.." }
async-trait = "0.1"
tokio = { version = "1.35", features = ["full"] }
# Add your messaging service SDK here
```

### Step 3: Implement the Adapter

In `adapters/my-messenger/src/lib.rs`:

```rust
use async_trait::async_trait;
use cli_chat_rs::{
    MessagingAdapter, AdapterResult, Message, Chat, Contact,
    MessageContent, ConnectionStatus, ChatId, ContactId, MessageId,
};
use tokio::sync::mpsc;

pub struct MyMessengerAdapter {
    status: ConnectionStatus,
    // Add your client/connection fields
}

impl MyMessengerAdapter {
    pub fn new(/* config parameters */) -> Self {
        Self {
            status: ConnectionStatus::Disconnected,
        }
    }
}

#[async_trait]
impl MessagingAdapter for MyMessengerAdapter {
    fn name(&self) -> &str {
        "My Messenger"
    }

    async fn connect(&mut self) -> AdapterResult<()> {
        // Initialize connection to your service
        self.status = ConnectionStatus::Connecting;

        // Your connection logic here

        self.status = ConnectionStatus::Connected;
        Ok(())
    }

    async fn disconnect(&mut self) -> AdapterResult<()> {
        // Clean up connection
        self.status = ConnectionStatus::Disconnected;
        Ok(())
    }

    fn connection_status(&self) -> ConnectionStatus {
        self.status
    }

    async fn get_chats(&self) -> AdapterResult<Vec<Chat>> {
        // Fetch chats from your service
        Ok(vec![])
    }

    async fn get_messages(&self, chat_id: &ChatId, limit: usize) -> AdapterResult<Vec<Message>> {
        // Fetch messages for a chat
        Ok(vec![])
    }

    async fn send_message(&mut self, chat_id: &ChatId, content: MessageContent) -> AdapterResult<Message> {
        // Send message through your service
        todo!()
    }

    async fn mark_as_read(&mut self, chat_id: &ChatId, message_id: &MessageId) -> AdapterResult<()> {
        // Mark message as read
        Ok(())
    }

    async fn get_contact(&self, contact_id: &ContactId) -> AdapterResult<Contact> {
        // Get contact info
        todo!()
    }

    async fn get_contacts(&self) -> AdapterResult<Vec<Contact>> {
        // Get all contacts
        Ok(vec![])
    }

    async fn subscribe_to_messages(&mut self) -> AdapterResult<mpsc::Receiver<Message>> {
        let (tx, rx) = mpsc::channel(100);

        // Set up listener for incoming messages
        // spawn a task that listens and sends to tx

        Ok(rx)
    }

    async fn search(&self, query: &str) -> AdapterResult<Vec<Chat>> {
        // Search for chats
        Ok(vec![])
    }
}
```

### Step 4: Update Workspace

In root `Cargo.toml`:

```toml
[workspace]
members = [".", "adapters/my-messenger"]

[dependencies]
my-messenger-adapter = { path = "adapters/my-messenger" }
```

### Step 5: Use Your Adapter

```rust
use my_messenger_adapter::MyMessengerAdapter;

let adapter = Box::new(MyMessengerAdapter::new());
let app = MessengerApp::new(config, adapter);
app.adapter_mut().connect().await?;
```

## Multiple Adapters

You can support multiple adapters by creating a factory pattern:

```rust
use cli_chat_rs::{Config, MessagingAdapter};

pub fn create_adapter(config: &Config) -> Box<dyn MessagingAdapter> {
    match config.active_adapter.as_str() {
        "demo" => Box::new(DemoAdapter::new()),
        _ => panic!("Unknown adapter: {}", config.active_adapter),
    }
}
```

## Best Practices

1. **Error Handling**: Always use `AdapterResult` and provide meaningful error messages
2. **Async Operations**: All I/O should be async using tokio
3. **State Management**: Keep connection state consistent
4. **Configuration**: Store adapter-specific config in the `settings` map
5. **Testing**: Write unit tests for your adapter
6. **Documentation**: Document adapter-specific features and requirements

## Troubleshooting

### Submodule Issues

```bash
# Update all submodules
git submodule update --init --recursive

# Pull latest changes in submodules
git submodule update --remote
```

### Build Issues

```bash
# Clean build
cargo clean
cargo build

# Update dependencies
cargo update
```

## Resources

- [Adapter Development Guide](ADAPTER_GUIDE.md)
