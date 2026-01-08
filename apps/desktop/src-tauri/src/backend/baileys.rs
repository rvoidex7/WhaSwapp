use async_trait::async_trait;
use tauri::{AppHandle, Emitter, Manager};
use std::process::Stdio;
use tokio::process::{Child, Command};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::backend::WhatsAppProvider;
use crate::utils::security::SecurityManager;
use crate::storage::{Storage, SqliteStorage};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct BaileysBackend {
    process: Arc<Mutex<Option<Child>>>,
    app_handle: AppHandle,
    stdin_tx: Arc<Mutex<Option<tokio::sync::mpsc::Sender<String>>>>,
    security: Arc<SecurityManager>,
    storage: Arc<SqliteStorage>,
    running: Arc<std::sync::atomic::AtomicBool>,
}

impl BaileysBackend {
    pub fn new(app_handle: AppHandle) -> Self {
        let security = app_handle.state::<Arc<SecurityManager>>().inner().clone();
        let master_key = security.get_master_key().map(hex::encode);

        let app_data_dir = app_handle.path().app_data_dir().unwrap_or(std::path::PathBuf::from("."));
        let db_path = app_data_dir.join("whaswapp.db");

        let storage = Arc::new(SqliteStorage::new(
            db_path.to_str().unwrap(),
            master_key.as_deref()
        ).expect("Failed to init DB"));

        Self {
            process: Arc::new(Mutex::new(None)),
            app_handle,
            stdin_tx: Arc::new(Mutex::new(None)),
            security,
            storage,
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    async fn merge_and_save_keys(&self, updates: &Value) -> anyhow::Result<()> {
        let key = "baileys_keys";
        let mut current_data = self.storage.get_auth_data(key).await
            .map_err(|e| anyhow::anyhow!("Storage error: {}", e))?
            .unwrap_or(serde_json::json!({}));

        if let Some(update_obj) = updates.as_object() {
            if let Some(current_obj) = current_data.as_object_mut() {
                for (type_key, type_val) in update_obj {
                    if !current_obj.contains_key(type_key) {
                        current_obj.insert(type_key.clone(), serde_json::json!({}));
                    }
                    if let Some(target_type_obj) = current_obj.get_mut(type_key).and_then(|v| v.as_object_mut()) {
                        if let Some(source_type_obj) = type_val.as_object() {
                            for (id, data) in source_type_obj {
                                if data.is_null() {
                                    target_type_obj.remove(id);
                                } else {
                                    target_type_obj.insert(id.clone(), data.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        self.storage.save_auth_data(key, &current_data).await
            .map_err(|e| anyhow::anyhow!("Storage error: {}", e))?;
        Ok(())
    }

    // Supervision loop
    async fn run_supervisor(&self) {
        self.running.store(true, std::sync::atomic::Ordering::Relaxed);

        while self.running.load(std::sync::atomic::Ordering::Relaxed) {
            println!("Baileys Supervisor: Starting process...");

            // Path resolution
            let resource_dir = self.app_handle.path().resource_dir().unwrap_or(std::path::PathBuf::from("."));
            let script_path = resource_dir.join("baileys-adapter/index.js");
            let script_path = if !script_path.exists() {
                 std::env::current_dir().unwrap().join("apps/desktop/src-tauri/baileys-adapter/index.js")
            } else {
                script_path
            };

            if !script_path.exists() {
                eprintln!("Baileys script not found, supervisor exiting.");
                break;
            }

            // Fetch latest auth data
            let creds = self.storage.get_auth_data("baileys_creds").await.unwrap_or(None);
            let keys = self.storage.get_auth_data("baileys_keys").await.unwrap_or(None);

            let mut child = match Command::new("node")
                .arg(&script_path)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::inherit())
                .kill_on_drop(true)
                .spawn() {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Failed to spawn Baileys: {}, retrying in 5s...", e);
                        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                        continue;
                    }
                };

            let stdin = child.stdin.take().unwrap();
            let stdout = child.stdout.take().unwrap();

            // Set up channels
            let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(32);
            *self.stdin_tx.lock().await = Some(tx.clone());
            *self.process.lock().await = Some(child);

            // Init command
            let init_cmd = IpcCommand {
                r#type: "init".to_string(),
                payload: serde_json::json!({
                    "auth_data": {
                        "creds": creds,
                        "keys": keys
                    }
                }),
            };
            if let Err(e) = tx.send(serde_json::to_string(&init_cmd).unwrap()).await {
                eprintln!("Failed to send init: {}", e);
            }

            // IO Loops
            let stdin_handle = tokio::spawn(async move {
                let mut stdin = stdin;
                while let Some(cmd) = rx.recv().await {
                    if stdin.write_all(cmd.as_bytes()).await.is_err() || stdin.write_all(b"\n").await.is_err() {
                        break;
                    }
                }
            });

            let app_handle = self.app_handle.clone();
            // We need a thread-safe way to call merge_and_save_keys.
            // Since we can't easily pass &self into this static future without cloning Arc<Self>,
            // we'll assume the storage is accessible.
            // Better: We clone the storage Arc.
            let storage = self.storage.clone();

            let stdout_handle = tokio::spawn(async move {
                let reader = BufReader::new(stdout);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    if let Ok(event) = serde_json::from_str::<IpcEvent>(&line) {
                        match event.r#type.as_str() {
                            "qr_code" => {
                                if let Some(c) = event.payload.as_str() { let _ = app_handle.emit("qr_code", c); }
                            }
                            "connection_status" => {
                                if let Some(s) = event.payload.as_str() { let _ = app_handle.emit("connection_status", s); }
                            }
                            "message" => {
                                let _ = app_handle.emit("message", format!("New Message: {:?}", event.payload));
                            }
                            "auth_update" => {
                                if let Some(update_type) = event.payload.get("type").and_then(|v| v.as_str()) {
                                    if let Some(data) = event.payload.get("data") {
                                        if update_type == "creds" {
                                            let _ = storage.save_auth_data("baileys_creds", data).await;
                                        } else if update_type == "keys" {
                                            // Implement merge logic here or helper
                                            // Re-implementing merge logic here locally to avoid `self` capture issues in spawn
                                            let key = "baileys_keys";
                                            if let Ok(current) = storage.get_auth_data(key).await {
                                                let mut current_data = current.unwrap_or(serde_json::json!({}));
                                                if let (Some(update_obj), Some(current_obj)) = (data.as_object(), current_data.as_object_mut()) {
                                                    for (type_key, type_val) in update_obj {
                                                        if !current_obj.contains_key(type_key) {
                                                            current_obj.insert(type_key.clone(), serde_json::json!({}));
                                                        }
                                                        if let (Some(target), Some(source)) = (current_obj.get_mut(type_key).and_then(|v| v.as_object_mut()), type_val.as_object()) {
                                                            for (id, val) in source {
                                                                if val.is_null() { target.remove(id); } else { target.insert(id.clone(), val.clone()); }
                                                            }
                                                        }
                                                    }
                                                }
                                                let _ = storage.save_auth_data(key, &current_data).await;
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            });

            // Wait for process exit
            let mut proc_guard = self.process.lock().await;
            if let Some(mut child) = proc_guard.take() {
                drop(proc_guard); // Release lock while waiting
                let _ = child.wait().await;
            } else {
                drop(proc_guard);
            }

            // Abort handles
            stdin_handle.abort();
            stdout_handle.abort();

            println!("Baileys process exited. Restarting in 1s...");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}

#[derive(Serialize)]
struct IpcCommand {
    r#type: String,
    payload: serde_json::Value,
}

#[derive(Deserialize)]
struct IpcEvent {
    r#type: String,
    payload: serde_json::Value,
}

#[async_trait]
impl WhatsAppProvider for BaileysBackend {
    async fn initialize(&self, _payload: String) -> anyhow::Result<()> {
        // Spawn the supervisor loop in the background
        // We need a way to clone self or access the logic.
        // Since we are inside &self (borrowed), we can't clone self if it's not Arc<Self>.
        // WhatsAppProvider trait takes &self.
        // But BaileysBackend struct fields are all Arcs. So we can clone the fields to a new struct or async block.

        let supervisor_backend = BaileysBackend {
            process: self.process.clone(),
            app_handle: self.app_handle.clone(),
            stdin_tx: self.stdin_tx.clone(),
            security: self.security.clone(),
            storage: self.storage.clone(),
            running: self.running.clone(),
        };

        tokio::spawn(async move {
            supervisor_backend.run_supervisor().await;
        });

        Ok(())
    }

    async fn send_message(&self, jid: String, content: String) -> anyhow::Result<()> {
        let tx_guard = self.stdin_tx.lock().await;
        if let Some(tx) = tx_guard.as_ref() {
             let cmd = IpcCommand {
                r#type: "send_message".to_string(),
                payload: serde_json::json!({
                    "jid": jid,
                    "content": content
                }),
            };
            tx.send(serde_json::to_string(&cmd)?).await?;
        }
        Ok(())
    }

    async fn disconnect(&self) -> anyhow::Result<()> {
        // Stop supervisor
        self.running.store(false, std::sync::atomic::Ordering::Relaxed);

        let tx_guard = self.stdin_tx.lock().await;
        if let Some(tx) = tx_guard.as_ref() {
             let cmd = IpcCommand {
                r#type: "disconnect".to_string(),
                payload: serde_json::json!({}),
            };
            let _ = tx.send(serde_json::to_string(&cmd)?).await;
        }

        // Force kill if needed
        let mut proc_guard = self.process.lock().await;
        if let Some(mut child) = proc_guard.take() {
            let _ = child.kill().await;
        }

        Ok(())
    }
}
