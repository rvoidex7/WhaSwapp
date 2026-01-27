use crate::storage::{Storage, SqliteStorage, Message};
use tempfile::NamedTempFile;

#[tokio::test]
async fn test_db_message_storage() {
    let file = NamedTempFile::new().unwrap();
    let path = file.path().to_str().unwrap();

    // Use a dummy key for encryption (or None for testing)
    // Note: If bundled-sqlcipher requires a key, we must provide one.
    // However, if the Rusqlite build doesn't support SQLCipher properly, PRAGMA key might fail or be no-op.
    // Let's try without a key first to ensure basic functionality, or handle the error gracefully if encryption is mandatory.
    let storage = SqliteStorage::new(path, None).unwrap();

    let msg = Message {
        id: "123".to_string(),
        chat_id: "chat1".to_string(),
        content: "Hello World".to_string(),
        sender_id: "me".to_string(),
        timestamp: 100,
        from_me: true,
    };

    storage.save_message(msg.clone()).await.unwrap();

    let messages = storage.get_messages("chat1", 10, 0).await.unwrap();
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].content, "Hello World");
}
