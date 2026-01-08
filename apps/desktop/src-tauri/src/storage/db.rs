use super::{Storage, Message, Chat};
use async_trait::async_trait;
use serde_json::Value;
use std::error::Error;
use std::sync::{Arc, Mutex};
use rusqlite::{params, Connection, OptionalExtension};

pub struct SqliteStorage {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteStorage {
    pub fn new(path: &str, key: Option<&str>) -> Result<Self, Box<dyn Error>> {
        let conn = Connection::open(path)?;

        // Encryption
        if let Some(k) = key {
            conn.execute(&format!("PRAGMA key = '{}';", k), [])?;
        }

        // Initialize Tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                chat_id TEXT NOT NULL,
                content TEXT NOT NULL,
                sender_id TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                from_me BOOLEAN NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS chats (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                unread_count INTEGER NOT NULL,
                last_message_timestamp INTEGER NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS auth_store (
                key TEXT PRIMARY KEY,
                data TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }
}

#[async_trait]
impl Storage for SqliteStorage {
    async fn save_message(&self, message: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO messages (id, chat_id, content, sender_id, timestamp, from_me)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                message.id,
                message.chat_id,
                message.content,
                message.sender_id,
                message.timestamp,
                message.from_me
            ],
        ).map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
        Ok(())
    }

    async fn get_messages(&self, chat_id: &str, limit: usize, offset: usize) -> Result<Vec<Message>, Box<dyn Error + Send + Sync>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, chat_id, content, sender_id, timestamp, from_me
             FROM messages
             WHERE chat_id = ?1
             ORDER BY timestamp DESC
             LIMIT ?2 OFFSET ?3"
        ).map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

        let message_iter = stmt.query_map(params![chat_id, limit, offset], |row| {
            Ok(Message {
                id: row.get(0)?,
                chat_id: row.get(1)?,
                content: row.get(2)?,
                sender_id: row.get(3)?,
                timestamp: row.get(4)?,
                from_me: row.get(5)?,
            })
        }).map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

        let mut messages = Vec::new();
        for msg in message_iter {
            messages.push(msg.map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?);
        }

        Ok(messages)
    }

    async fn save_chat(&self, chat: Chat) -> Result<(), Box<dyn Error + Send + Sync>> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO chats (id, name, unread_count, last_message_timestamp)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                chat.id,
                chat.name,
                chat.unread_count,
                chat.last_message_timestamp
            ],
        ).map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
        Ok(())
    }

    async fn get_chats(&self) -> Result<Vec<Chat>, Box<dyn Error + Send + Sync>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, unread_count, last_message_timestamp FROM chats ORDER BY last_message_timestamp DESC"
        ).map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

        let chat_iter = stmt.query_map([], |row| {
            Ok(Chat {
                id: row.get(0)?,
                name: row.get(1)?,
                unread_count: row.get(2)?,
                last_message_timestamp: row.get(3)?,
            })
        }).map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

        let mut chats = Vec::new();
        for chat in chat_iter {
            chats.push(chat.map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?);
        }

        Ok(chats)
    }

    async fn save_auth_data(&self, key: &str, data: &Value) -> Result<(), Box<dyn Error + Send + Sync>> {
        let conn = self.conn.lock().unwrap();
        let json_str = serde_json::to_string(data).map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

        conn.execute(
            "INSERT OR REPLACE INTO auth_store (key, data) VALUES (?1, ?2)",
            params![key, json_str],
        ).map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
        Ok(())
    }

    async fn get_auth_data(&self, key: &str) -> Result<Option<Value>, Box<dyn Error + Send + Sync>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT data FROM auth_store WHERE key = ?1")
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

        let result: Option<String> = stmt.query_row(params![key], |row| row.get(0))
            .optional()
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

        match result {
            Some(json_str) => {
                let val: Value = serde_json::from_str(&json_str)
                    .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
                Ok(Some(val))
            },
            None => Ok(None),
        }
    }

    async fn remove_auth_data(&self, key: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM auth_store WHERE key = ?1", params![key])
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
        Ok(())
    }
}
