# AGENTS.md

This file serves as a guide for AI agents and human developers working on the WhaSwapp project. It outlines the architectural philosophy, technical standards, and specific instructions for navigating the codebase.

## 1. Project Philosophy: The Manifesto
WhaSwapp is a rebellion against the resource-heavy, privacy-invasive official WhatsApp desktop client.
*   **Privacy First:** All data is stored locally in an encrypted SQLite database. No unauthorized cloud syncs.
*   **Modularity:** We wrap existing libraries (`whatsapp-rust`, `Baileys`, `whatsapp-web.js`) using an Adapter Pattern.
*   **Performance:** The core logic is in Rust. The UI is lightweight (Tauri/TUI).

## 2. Architecture Overview
The project is a monorepo managed by `pnpm`.

*   `apps/desktop`: The main Tauri application.
    *   `src-tauri`: Rust backend. Acts as the "Supervisor".
    *   `src`: React frontend (for Desktop mode).
*   `libs/`: Shared libraries.
    *   `cli-chat-rs`: Responsive TUI library.
    *   `web-intelligence`: Browser automation tools.
*   `whatsapp-rust`, `Baileys`, `whatsapp-web.js`: Git submodules containing the protocol implementations.

### The Adapter Pattern
We define a `WhatsAppProvider` trait in Rust. Each backend implements this trait:
*   `RustBackend`: Direct FFI/Native binding to `whatsapp-rust`.
*   `BaileysBackend`: Spawns a Node.js sidecar and communicates via JSON-RPC over Stdio. Credentials are synced to Rust memory/DB, never written to disk by Node.

## 3. Technical Standards

### State Management
*   **Frontend:** Zustand (`useFeatureStore`, `useChatStore`).
*   **Backend:** Rust structs wrapped in `Arc<Mutex<...>>` or `RwLock`.
*   **Persistence:** `SqliteStorage` (encrypted with SQLCipher).

### Async Discipline
*   Do not block the Tokio runtime. Use `tokio::task::spawn_blocking` for heavy synchronous operations (like DB writes or file I/O).
*   Properly supervise child processes (`Baileys`). Restart them if they crash.

### Error Handling
*   Use `anyhow::Result` for application-level errors.
*   Log errors with context. Do not silence critical failures.

## 4. Dependency Strategy: Fork & Patch
We use "Fork & Patch" for upstream libraries.
1.  **Submodules:** We keep pointers to upstream repos.
2.  **Patches:** If we need a feature (e.g., `register_handler`), we patch it locally.
3.  **Documentation:** All patches must be documented in `PATCHES.md` and ideally submitted upstream.
4.  **Automation:** Use `scripts/apply_patches.sh` to apply local modifications after submodule updates.

## 5. Development Tips
*   **Build:** `pnpm tauri dev`
*   **Termux:** Build `cli-chat-rs` directly on the device.
*   **Security:** Always ensure `bundled-sqlcipher` is enabled in Cargo features.

## 6. Known Issues / To-Do
*   `whatsapp-rust`: Encryption support in `SqliteStore` constructor varies by version. We currently use a local patch for event handling.
*   `Baileys`: Reconnection logic requires a process restart to ensure fresh key state from the DB.

---
*Created by the WhaSwapp Development Team (Human & AI)*
