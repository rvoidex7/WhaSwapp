#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod backend;
mod commands;
mod utils;
mod storage;

use tauri::Manager;
use backend::WhatsAppManager;
use std::io::{self, Write};
use std::process;
use utils::security::SecurityManager;
use std::sync::Arc;

fn main() {
    // Determine App Data Directory (Mocking for CLI)
    // We need a stable place to store security.json *before* Tauri launches
    // On Windows: %APPDATA%\com.whaswapp.app
    // On Linux: ~/.local/share/com.whaswapp.app
    // For simplicity, we'll rely on `dirs::data_dir` + bundle identifier.
    let app_data_dir = dirs::data_dir()
        .map(|p| p.join("com.whaswapp.app"))
        .expect("Could not determine app data dir");

    if !app_data_dir.exists() {
        std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data dir");
    }

    let security = Arc::new(SecurityManager::new(app_data_dir.clone()));

    println!("Welcome to WhaSwapp!");
    println!("--------------------");

    // Security Check
    if security.is_configured() {
        println!("Locked. Please enter startup password.");
        loop {
            print!("Password: ");
            io::stdout().flush().unwrap();

            let password = rpassword::read_password().unwrap();
            match security.unlock(&password) {
                Ok(true) => {
                    println!("Unlocked!");
                    break;
                },
                Ok(false) => println!("Incorrect password. Try again."),
                Err(e) => {
                    println!("Error: {}. Exiting.", e);
                    process::exit(1);
                }
            }
        }
    } else {
        println!("No startup password set.");
        print!("Create one? [Y/n]: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().eq_ignore_ascii_case("y") || input.trim().is_empty() {
             loop {
                print!("New Password: ");
                io::stdout().flush().unwrap();
                let p1 = rpassword::read_password().unwrap();
                print!("Confirm Password: ");
                io::stdout().flush().unwrap();
                let p2 = rpassword::read_password().unwrap();

                if p1 == p2 {
                    security.init(&p1).expect("Failed to initialize security");
                    println!("Password set and unlocked.");
                    break;
                } else {
                    println!("Passwords do not match. Try again.");
                }
             }
        }
    }

    println!("--------------------");
    println!("Select Backend:");
    println!("1. Rust Native (Default)");
    println!("2. Baileys (Node.js)");
    println!("3. whatsapp-web.js");
    print!("Selection [1]: ");
    io::stdout().flush().unwrap();

    let mut backend_input = String::new();
    io::stdin().read_line(&mut backend_input).unwrap();
    let backend = match backend_input.trim() {
        "2" => "baileys",
        "3" => "wwebjs",
        _ => "rust",
    };

    println!("\nSelect Frontend:");
    println!("1. Desktop (Tauri) (Default)");
    println!("2. Terminal (TUI)");
    println!("3. Browser (Chrome/Edge)");
    print!("Selection [1]: ");
    io::stdout().flush().unwrap();

    let mut frontend_input = String::new();
    io::stdin().read_line(&mut frontend_input).unwrap();
    let frontend = match frontend_input.trim() {
        "2" => "tui",
        "3" => "browser",
        _ => "tauri",
    };

    println!("\nLaunching WhaSwapp with Backend: [{}] and Frontend: [{}]...", backend, frontend);

    if frontend == "tui" {
        // Launch TUI mode (Placeholder for cli-chat-rs integration)
        println!("Starting TUI mode... (Not implemented yet, exiting)");
        process::exit(0);
    } else if frontend == "browser" {
         // Launch Browser mode (Placeholder for web-intelligence integration)
         println!("Starting Browser mode... (Not implemented yet, exiting)");
         process::exit(0);
    }

    let backend_config = backend.to_string();
    let frontend_config = frontend.to_string();

    // Default: Launch Tauri
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            let manager = WhatsAppManager::new();

            // Pass the SecurityManager instance to Tauri state
            app.manage(security);
            app.manage(manager);

            app.manage(SessionConfig {
                backend: backend_config,
                frontend: frontend_config
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::setup_session,
            commands::send_message,
            commands::reset_session,
            commands::get_session_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub struct SessionConfig {
    pub backend: String,
    pub frontend: String,
}
