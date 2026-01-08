use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::config::ShortcutConfig;

/// Application actions that can be triggered by keyboard shortcuts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    NextChat,
    PrevChat,
    SendMessage,
    Search,
    ToggleSidebar,
    ScrollUp,
    ScrollDown,
    PageUp,
    PageDown,
    None,
}

/// Keyboard shortcut handler
pub struct KeyboardHandler {
    #[allow(dead_code)] // Reserved for future dynamic key mapping feature
    config: ShortcutConfig,
}

impl KeyboardHandler {
    pub fn new(config: ShortcutConfig) -> Self {
        Self { config }
    }

    /// Map a key event to an action based on configured shortcuts
    /// Note: Currently uses hardcoded mappings matching the default config.
    /// Future enhancement will support fully dynamic key mapping.
    pub fn handle_key(&self, key: KeyEvent) -> Action {
        match (key.code, key.modifiers) {
            // Ctrl+Q - Quit
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => Action::Quit,
            (KeyCode::Char('Q'), KeyModifiers::CONTROL) => Action::Quit,

            // Ctrl+N - Next chat
            (KeyCode::Char('n'), KeyModifiers::CONTROL) => Action::NextChat,
            (KeyCode::Char('N'), KeyModifiers::CONTROL) => Action::NextChat,

            // Ctrl+P - Previous chat
            (KeyCode::Char('p'), KeyModifiers::CONTROL) => Action::PrevChat,
            (KeyCode::Char('P'), KeyModifiers::CONTROL) => Action::PrevChat,

            // Enter - Send message
            (KeyCode::Enter, KeyModifiers::NONE) => Action::SendMessage,

            // Ctrl+F - Search
            (KeyCode::Char('f'), KeyModifiers::CONTROL) => Action::Search,
            (KeyCode::Char('F'), KeyModifiers::CONTROL) => Action::Search,

            // Ctrl+L - Toggle sidebar
            (KeyCode::Char('l'), KeyModifiers::CONTROL) => Action::ToggleSidebar,
            (KeyCode::Char('L'), KeyModifiers::CONTROL) => Action::ToggleSidebar,

            // Arrow keys - Scroll
            (KeyCode::Up, KeyModifiers::NONE) => Action::ScrollUp,
            (KeyCode::Down, KeyModifiers::NONE) => Action::ScrollDown,

            // Page Up/Down
            (KeyCode::PageUp, KeyModifiers::NONE) => Action::PageUp,
            (KeyCode::PageDown, KeyModifiers::NONE) => Action::PageDown,

            _ => Action::None,
        }
    }

    /// Get the configured shortcut description for display
    pub fn get_shortcuts_help(&self) -> Vec<(String, String)> {
        vec![
            (self.config.quit.clone(), "Quit application".to_string()),
            (self.config.next_chat.clone(), "Next chat".to_string()),
            (self.config.prev_chat.clone(), "Previous chat".to_string()),
            (self.config.send_message.clone(), "Send message".to_string()),
            (self.config.search.clone(), "Search".to_string()),
            (self.config.toggle_sidebar.clone(), "Toggle sidebar".to_string()),
            (self.config.scroll_up.clone(), "Scroll up".to_string()),
            (self.config.scroll_down.clone(), "Scroll down".to_string()),
        ]
    }
}
