use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::Result;

/// Main configuration for the messenger CLI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Active adapter to use
    pub active_adapter: String,

    /// Adapter-specific configurations
    pub adapters: HashMap<String, AdapterConfig>,

    /// UI/Keyboard shortcuts configuration
    pub shortcuts: ShortcutConfig,

    /// General application settings
    pub app: AppConfig,
}

/// Configuration for a specific messaging adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConfig {
    /// Whether this adapter is enabled
    pub enabled: bool,

    /// Adapter-specific settings (flexible for different adapters)
    pub settings: HashMap<String, serde_json::Value>,
}

/// Keyboard shortcut configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutConfig {
    pub quit: String,
    pub next_chat: String,
    pub prev_chat: String,
    pub send_message: String,
    pub search: String,
    pub toggle_sidebar: String,
    pub scroll_up: String,
    pub scroll_down: String,
    pub page_up: String,
    pub page_down: String,
}

impl Default for ShortcutConfig {
    fn default() -> Self {
        Self {
            quit: "Ctrl+Q".to_string(),
            next_chat: "Ctrl+N".to_string(),
            prev_chat: "Ctrl+P".to_string(),
            send_message: "Enter".to_string(),
            search: "Ctrl+F".to_string(),
            toggle_sidebar: "Ctrl+L".to_string(),
            scroll_up: "Up".to_string(),
            scroll_down: "Down".to_string(),
            page_up: "PageUp".to_string(),
            page_down: "PageDown".to_string(),
        }
    }
}

/// General application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Path to store application data
    pub data_dir: PathBuf,

    /// Log level
    pub log_level: String,

    /// Maximum messages to load per chat
    pub messages_per_chat: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            data_dir: dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".cli-chat-rs"),
            log_level: "info".to_string(),
            messages_per_chat: 50,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            active_adapter: "demo".to_string(),
            adapters: HashMap::new(),
            shortcuts: ShortcutConfig::default(),
            app: AppConfig::default(),
        }
    }
}

impl Config {
    /// Load configuration from a file
    pub fn load(path: &PathBuf) -> Result<Self> {
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            let config: Config = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    /// Save configuration to a file
    pub fn save(&self, path: &PathBuf) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
