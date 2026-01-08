# cli-chat-rs

A modular TUI (Text User Interface) chat client written in Rust. Designed to be protocol-agnostic, supporting plugins for various messaging services.

## Overview

`cli-chat-rs` is a **platform-agnostic** messaging client that can integrate with any messaging service through a simple adapter interface. It provides a consistent CLI/TUI experience regardless of the underlying messaging platform.

## Features

- 🔌 **Plugin Architecture**: Easy-to-implement adapter trait for any messaging service
- ⌨️ **Keyboard Shortcuts**: Customizable key bindings for efficient navigation
- 🎨 **Terminal UI**: Beautiful TUI built with ratatui
- 🔄 **Async First**: Built on tokio for efficient async operations
- 📦 **Flexible Integration**: Support for submodules, workspace members, or dependencies
- ⚙️ **Configurable**: JSON-based configuration system

## Quick Start

### Installation

```bash
git clone https://github.com/rvoidex7/cli-chat-rs
cd cli-chat-rs
cargo build --release
```

### Run with Demo Adapter

```bash
cargo run --release
```

## Integration Methods

### As a Submodule

Integrate existing messaging service implementations:

```bash
# Example: Add your custom adapter
git submodule add https://github.com/youruser/your-adapter adapters/your-adapter
```

### As a Workspace Member

Add your adapter to the workspace in `Cargo.toml`:

```toml
[workspace]
members = ["adapters/your-adapter"]
```

### As a Dependency

```toml
[dependencies]
cli-chat-rs = "0.1"
```

## Creating Custom Adapters

See [ADAPTER_GUIDE.md](ADAPTER_GUIDE.md) for detailed instructions on creating adapters for your messaging service.

### Quick Example

```rust
use async_trait::async_trait;
use cli_chat_rs::{MessagingAdapter, AdapterResult, /* ... */};

pub struct YourAdapter { /* ... */ }

#[async_trait]
impl MessagingAdapter for YourAdapter {
    fn name(&self) -> &str { "Your Service" }

    async fn connect(&mut self) -> AdapterResult<()> {
        // Connect to your service
        Ok(())
    }

    // Implement other required methods...
}
```

## Keyboard Shortcuts

| Key       | Action           |
|-----------|------------------|
| Ctrl+Q    | Quit             |
| Ctrl+N    | Next chat        |
| Ctrl+P    | Previous chat    |
| Enter     | Send message     |
| Ctrl+F    | Search           |
| Ctrl+L    | Toggle sidebar   |
| Ctrl+H    | Show/hide help   |
| ↑/↓       | Scroll messages  |

All shortcuts are customizable via the configuration file.

## Configuration

Create `~/.cli-chat-rs/config.json`:

```json
{
  "active_adapter": "demo",
  "adapters": {
    "demo": {
      "enabled": true,
      "settings": {}
    }
  },
  "shortcuts": {
    "quit": "Ctrl+Q",
    "next_chat": "Ctrl+N",
    "prev_chat": "Ctrl+P",
    "send_message": "Enter",
    "search": "Ctrl+F",
    "toggle_sidebar": "Ctrl+L"
  },
  "app": {
    "log_level": "info",
    "messages_per_chat": 50
  }
}
```

## Architecture

```
cli-chat-rs/
├── src/
│   ├── adapter/          # Adapter trait and implementations
│   │   ├── traits.rs     # Core MessagingAdapter trait
│   │   └── demo.rs       # Demo adapter implementation
│   ├── types/            # Common data types
│   ├── config/           # Configuration management
│   └── ui/               # Terminal UI and keyboard handling
├── adapters/             # Third-party adapters (submodules)
│   └── your-adapter/
└── examples/             # Example implementations
```

## Acknowledgments

- Built with [ratatui](https://github.com/ratatui-org/ratatui) for the terminal interface
- Thanks to the Rust community for amazing async libraries

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT
