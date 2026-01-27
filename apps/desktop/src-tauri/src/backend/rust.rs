use async_trait::async_trait;
use tauri::{AppHandle, Emitter, Manager};
use std::sync::Arc;
use tokio::sync::Mutex;
use whatsapp_rust::{Client, store::SqliteStore};
use whatsapp_rust::store::persistence_manager::PersistenceManager;
use whatsapp_rust::transport::{TokioWebSocketTransportFactory, UreqHttpClient};
use whatsapp_rust::waproto::whatsapp as wa;
use wacore_binary::jid::Jid;
use whatsapp_rust::types::events::{Event, EventHandler};
use crate::backend::WhatsAppProvider;
use crate::utils::security::SecurityManager;
use std::str::FromStr;

pub struct RustBackend {
    client: Arc<Mutex<Option<Arc<Client>>>>,
    app_handle: AppHandle,
    security: Arc<SecurityManager>,
}

impl RustBackend {
    pub fn new(app_handle: AppHandle) -> Self {
        let security = app_handle.state::<Arc<SecurityManager>>().inner().clone();
        Self {
            client: Arc::new(Mutex::new(None)),
            app_handle,
            security,
        }
    }
}

struct TauriEventHandler {
    app_handle: AppHandle,
}

impl EventHandler for TauriEventHandler {
    fn handle_event(&self, event: &Event) {
         match event {
            Event::PairingQrCode { code, .. } => {
                let _ = self.app_handle.emit("qr_code", code);
            }
            Event::Connected(_) => {
                let _ = self.app_handle.emit("connection_status", "connected");
            }
            Event::Disconnected(_) => {
                 let _ = self.app_handle.emit("connection_status", "disconnected");
            }
            Event::Message(_msg, _info) => {
                 // Simplified event emission
                 // We should emit a JSON representation of the message
                 // For now, just a notification string to verify connectivity
                 // msg is Box<wa::Message>, info is MessageInfo
                 // MessageInfo has a source field which contains the sender
                 let _ = self.app_handle.emit("message", "New Message".to_string());
            }
            _ => {}
         }
    }
}

#[async_trait]
impl WhatsAppProvider for RustBackend {
    async fn initialize(&self, _payload: String) -> anyhow::Result<()> {
        let app_data_dir = self.app_handle.path().app_data_dir()?;
        if !app_data_dir.exists() {
            std::fs::create_dir_all(&app_data_dir)?;
        }
        let db_path = app_data_dir.join("session.db");
        let db_url = format!("sqlite:{}?mode=rwc", db_path.to_string_lossy());

        // Get Master Key for encryption
        let _master_key = self.security.get_master_key()
            .map(hex::encode);

        // Encryption support removed/changed in recent whatsapp-rust versions or not exposed this way
        // Fallback to standard new()
        let store = Arc::new(SqliteStore::new(&db_url).await?);

        let pm = Arc::new(PersistenceManager::new(store).await?);

        let transport = Arc::new(TokioWebSocketTransportFactory::new());
        let http = Arc::new(UreqHttpClient);

        let (client, _sync_tasks) = Client::new(pm, transport, http, None).await;

        {
            let mut c = self.client.lock().await;
            *c = Some(client.clone());
        }

        // Register handler
        let handler = Arc::new(TauriEventHandler {
           app_handle: self.app_handle.clone(),
        });
        client.register_handler(handler);

        // Start the client
        let c = client.clone();
        tokio::spawn(async move {
            c.run().await;
        });

        Ok(())
    }

    async fn send_message(&self, jid: String, content: String) -> anyhow::Result<()> {
        let client_guard = self.client.lock().await;
        if let Some(client) = client_guard.as_ref() {
             let jid = Jid::from_str(&jid).map_err(|e| anyhow::anyhow!("Invalid JID: {}", e))?;

             let message = wa::Message {
                 conversation: Some(content),
                 ..Default::default()
             };

             client.send_message(jid, message).await?;
        } else {
            return Err(anyhow::anyhow!("Client not initialized"));
        }
        Ok(())
    }

    async fn disconnect(&self) -> anyhow::Result<()> {
        let mut client_guard = self.client.lock().await;
        if let Some(client) = client_guard.take() {
            client.disconnect().await;
        }
        Ok(())
    }
}
