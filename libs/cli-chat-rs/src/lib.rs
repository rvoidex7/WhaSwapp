pub mod adapter;
pub mod config;
pub mod types;
pub mod ui;

pub use adapter::{MessagingAdapter, AdapterResult, DemoAdapter};
pub use config::Config;
pub use types::*;
pub use ui::{Action, KeyboardHandler};

/// Application state manager
pub struct MessengerApp {
    config: Config,
    adapter: Box<dyn MessagingAdapter>,
}

impl MessengerApp {
    /// Create a new messenger application with the given adapter
    pub fn new(config: Config, adapter: Box<dyn MessagingAdapter>) -> Self {
        Self { config, adapter }
    }

    /// Get reference to the current adapter
    pub fn adapter(&self) -> &dyn MessagingAdapter {
        self.adapter.as_ref()
    }

    /// Get mutable reference to the current adapter
    pub fn adapter_mut(&mut self) -> &mut dyn MessagingAdapter {
        self.adapter.as_mut()
    }

    /// Get reference to the configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get mutable reference to the configuration
    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }
}
