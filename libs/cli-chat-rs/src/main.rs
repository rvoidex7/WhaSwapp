use cli_chat_rs::{Config, DemoAdapter, MessengerApp, KeyboardHandler, Action};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;

/// Active screen state for mobile/narrow view
#[derive(PartialEq)]
enum ActiveScreen {
    ChatList,
    ChatView,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let config_path = std::env::var("CLI_CHAT_CONFIG")
        .unwrap_or_else(|_| {
            dirs::home_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join(".cli-chat-rs")
                .join("config.json")
                .to_string_lossy()
                .to_string()
        });

    let config = Config::load(&std::path::PathBuf::from(&config_path))
        .unwrap_or_else(|_| Config::default());

    // Create demo adapter (in a real app, this would be selected based on config)
    let adapter = Box::new(DemoAdapter::new());
    let mut app = MessengerApp::new(config, adapter);

    // Connect to the messaging service
    println!("Connecting to {}...", app.adapter().name());
    app.adapter_mut().connect().await.map_err(|e| format!("Connection error: {}", e))?;
    println!("Connected!");

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create keyboard handler
    let keyboard_handler = KeyboardHandler::new(app.config().shortcuts.clone());

    // Run the UI
    let result = run_ui(&mut terminal, &mut app, &keyboard_handler).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

async fn run_ui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut MessengerApp,
    keyboard_handler: &KeyboardHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut selected_chat = 0;
    let mut input_message = String::new();
    let mut show_help = false;
    let mut active_screen = ActiveScreen::ChatList;

    // Threshold for switching to mobile layout (columns)
    const MOBILE_THRESHOLD: u16 = 80;

    loop {
        // Get chats
        let chats = app.adapter().get_chats().await.map_err(|e| format!("Failed to get chats: {}", e))?;

        terminal.draw(|f| {
            let size = f.size();
            let is_mobile = size.width < MOBILE_THRESHOLD;

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(1), // Content
                    Constraint::Length(if !is_mobile || active_screen == ActiveScreen::ChatView { 3 } else { 0 }), // Input (hide in chat list on mobile)
                    Constraint::Length(1), // Status bar
                ])
                .split(size);

            let content_area = chunks[0];
            let input_area = chunks[1];
            let status_area = chunks[2];

            // Calculate layout based on available width
            let (chat_list_area, message_area) = if is_mobile {
                match active_screen {
                    ActiveScreen::ChatList => (content_area, Rect::default()),
                    ActiveScreen::ChatView => (Rect::default(), content_area),
                }
            } else {
                // Desktop: Split view
                let split = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                    .split(content_area);
                (split[0], split[1])
            };

            // Render Chat List (if visible)
            if chat_list_area.width > 0 {
                let chat_items: Vec<ListItem> = chats
                    .iter()
                    .enumerate()
                    .map(|(i, chat)| {
                        let style = if i == selected_chat {
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default()
                        };

                        let unread = if chat.unread_count > 0 {
                            format!(" ({})", chat.unread_count)
                        } else {
                            String::new()
                        };

                        ListItem::new(format!("{}{}", chat.name, unread)).style(style)
                    })
                    .collect();

                let chat_list = List::new(chat_items)
                    .block(Block::default().borders(Borders::ALL).title("Chats"));
                f.render_widget(chat_list, chat_list_area);
            }

            // Render Message Area (if visible)
            if message_area.width > 0 {
                let messages_block = Block::default()
                    .borders(Borders::ALL)
                    .title(if selected_chat < chats.len() {
                        chats[selected_chat].name.clone()
                    } else {
                        "No chat selected".to_string()
                    });

                let welcome_text = if show_help {
                    let shortcuts = keyboard_handler.get_shortcuts_help();
                    let lines: Vec<Line> = shortcuts
                        .iter()
                        .map(|(key, desc)| {
                            Line::from(vec![
                                Span::styled(format!("{:15}", key), Style::default().fg(Color::Cyan)),
                                Span::raw(desc.clone()),
                            ])
                        })
                        .collect();
                    Paragraph::new(lines).block(messages_block)
                } else {
                    Paragraph::new(format!(
                        "Welcome to CLI Chat RS!\n\n\
                        Connected to: {}\n\n\
                        Press Ctrl+H for help\n\
                        Press Ctrl+Q to quit\n\
                        {}",
                        app.adapter().name(),
                        if is_mobile { "Press ESC to go back" } else { "" }
                    ))
                    .block(messages_block)
                };

                f.render_widget(welcome_text, message_area);
            }

            // Render Input (if visible)
            if input_area.height > 0 {
                let input = Paragraph::new(input_message.as_str())
                    .block(Block::default().borders(Borders::ALL).title("Message"));
                f.render_widget(input, input_area);
            }

            // Status bar
            let status = Paragraph::new(format!(
                "Adapter: {} | Status: {:?} | {} | {}",
                app.adapter().name(),
                app.adapter().connection_status(),
                if is_mobile { if active_screen == ActiveScreen::ChatList { "Mobile: List" } else { "Mobile: Chat" } } else { "Desktop" },
                "Ctrl+Q: Quit"
            ))
            .style(Style::default().bg(Color::Blue).fg(Color::White));
            f.render_widget(status, status_area);
        })?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let action = keyboard_handler.handle_key(key);
                let size = terminal.size()?;
                let is_mobile = size.width < MOBILE_THRESHOLD;

                // Handle ESC for mobile back navigation
                if is_mobile && key.code == KeyCode::Esc {
                    if active_screen == ActiveScreen::ChatView {
                        active_screen = ActiveScreen::ChatList;
                        continue;
                    }
                }

                match action {
                    Action::Quit => break,
                    Action::NextChat => {
                        if !chats.is_empty() {
                            selected_chat = (selected_chat + 1) % chats.len();
                        }
                    }
                    Action::PrevChat => {
                        if !chats.is_empty() {
                            selected_chat = if selected_chat == 0 {
                                chats.len() - 1
                            } else {
                                selected_chat - 1
                            };
                        }
                    }
                    Action::SendMessage => {
                        // On mobile, Enter on ChatList enters the chat
                        if is_mobile && active_screen == ActiveScreen::ChatList {
                            active_screen = ActiveScreen::ChatView;
                        } else if !input_message.is_empty() && selected_chat < chats.len() {
                            let content = cli_chat_rs::MessageContent::Text(input_message.clone());
                            let _ = app.adapter_mut().send_message(&chats[selected_chat].id, content).await;
                            input_message.clear();
                        }
                    }
                    _ => {}
                }

                // Handle text input
                // Only allow typing if we are in ChatView (on mobile) or always on desktop
                if !is_mobile || active_screen == ActiveScreen::ChatView {
                    if let KeyCode::Char(c) = key.code {
                        if !key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                            input_message.push(c);
                        } else if c == 'h' || c == 'H' {
                            show_help = !show_help;
                        }
                    } else if let KeyCode::Backspace = key.code {
                        input_message.pop();
                    }
                }
            }
        }
    }

    Ok(())
}
