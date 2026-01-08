use tauri::{AppHandle, State};
use tokio::sync::Mutex;
use crate::backend::WhatsAppProvider;
use crate::backend::rust::RustBackend;
use crate::backend::baileys::BaileysBackend;
use crate::utils::chrome::launch_chrome;
use crate::SessionConfig; // Import from main

pub struct WhatsAppManager {
    pub provider: Mutex<Option<Box<dyn WhatsAppProvider>>>,
}

impl WhatsAppManager {
    pub fn new() -> Self {
        Self {
            provider: Mutex::new(None),
        }
    }
}

// Commands

#[tauri::command]
pub async fn get_session_config(config: State<'_, SessionConfig>) -> Result<SessionConfigResponse, String> {
    Ok(SessionConfigResponse {
        backend: config.backend.clone(),
        frontend: config.frontend.clone(),
    })
}

#[derive(serde::Serialize)]
pub struct SessionConfigResponse {
    backend: String,
    frontend: String,
}

#[tauri::command]
pub async fn setup_session(
    app: AppHandle,
    manager: State<'_, WhatsAppManager>,
    backend: String,
    frontend: String,
) -> Result<(), String> {
    println!("Setting up session: Backend={}, Frontend={}", backend, frontend);

    // Handle Frontend Selection
    if frontend == "chrome" {
        // Launch Chrome in Joker mode
        if let Err(e) = launch_chrome("") {
            return Err(format!("Failed to launch Chrome: {}", e));
        }
    }

    // Handle Backend Selection
    let mut provider_lock = manager.provider.lock().await;

    // Disconnect existing if any
    if let Some(p) = provider_lock.take() {
        let _ = p.disconnect().await;
    }

    if backend == "baileys" {
        println!("Baileys backend selected. Initializing Baileys Adapter...");
        let backend = BaileysBackend::new(app.clone());
        if let Err(e) = backend.initialize("".to_string()).await {
             eprintln!("Failed to initialize Baileys backend: {}", e);
             return Err(format!("Failed to initialize Baileys backend: {}", e));
        }
        *provider_lock = Some(Box::new(backend));
        println!("Baileys backend initialized successfully.");
    } else if backend == "rust" {
        println!("Rust backend selected. Initializing Rust Adapter...");
        let backend = RustBackend::new(app.clone());
        if let Err(e) = backend.initialize("".to_string()).await {
            eprintln!("Failed to initialize Rust backend: {}", e);
            return Err(format!("Failed to initialize Rust backend: {}", e));
        }
        *provider_lock = Some(Box::new(backend));
        println!("Rust backend initialized successfully.");
    } else if backend == "wwebjs" || backend == "whatsapp-web.js" {
        println!("WhatsApp-Web.js backend selected (Adapter to be implemented).");
    } else {
        return Err("Unsupported backend".to_string());
    }

    Ok(())
}

#[tauri::command]
pub async fn send_message(
    manager: State<'_, WhatsAppManager>,
    jid: String,
    content: String,
) -> Result<(), String> {
    let provider_lock = manager.provider.lock().await;

    if let Some(provider) = provider_lock.as_ref() {
        provider.send_message(jid, content).await.map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("No active session".to_string())
    }
}

#[tauri::command]
pub async fn reset_session(
    manager: State<'_, WhatsAppManager>,
) -> Result<(), String> {
    let mut provider_lock = manager.provider.lock().await;

    if let Some(provider) = provider_lock.take() {
        provider.disconnect().await.map_err(|e| e.to_string())?;
    }

    Ok(())
}
