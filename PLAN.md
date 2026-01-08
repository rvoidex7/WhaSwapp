# WhaSwapp Master Plan

This document serves as the central To-Do list and roadmap for the project.

## 🏗️ Phase 1: Foundation (Complete)
- [x] **Cleanup:** Remove old backend implementations (`baileys.rs`, `wwebjs.rs`, etc.).
- [x] **Rebranding:** Rename project to "WhaSwapp" (Configs, Cargo.toml, Package.json).
- [x] **Structure:** Establish Monorepo structure (`apps/desktop` + `libs/`).
- [x] **Dependencies:** Integrate `cli-chat-rs` and `web-intelligence` as local libraries.
- [x] **Submodules:** Re-add `Baileys`, `whatsapp-web.js`, and `whatsapp-rust`.

## 🧠 Phase 2: Core Logic & CLI Launcher (Completed)
- [x] **CLI Entry Point:** Refactor `main.rs` to start in a Terminal interface (TUI) instead of opening a window immediately.
- [x] **Launcher Logic:** Implement the backend/frontend selector using `cli-chat-rs` components (or std::io).
- [x] **Adapter Implementation:** Connect the submodules to the Core.
    - [x] **Rust Native:** Implement `WhatsAppProvider` using the `whatsapp-rust` crate.
    - [x] **Baileys:** Implement `WhatsAppProvider` via a Node.js sidecar/IPC bridge.
    - [ ] **WWebJS:** Implement `WhatsAppProvider` via a Node.js sidecar/IPC bridge.
- [x] **Session Management:** Finalize `setup_session` to initialize the selected adapter.

## ✨ Phase 3: Feature Implementation (Future)
- [ ] **Ghost Mode:**
    - [ ] Implement `read_receipt` interception in Rust/Node backends.
    - [ ] Add toggle in UI Settings.
- [ ] **Anti-Delete:**
    - [ ] Handle `protocol_message` (Revoke) events.
    - [ ] Update React Store to flag revoked messages instead of deleting them.
- [ ] **Local AI:**
    - [ ] Connect `web-intelligence` output to the React frontend.
    - [ ] Implement "Summarize" button in Chat UI.
- [ ] **UI Customization:**
    - [ ] Implement Theme Switcher.
    - [ ] Implement "Save View Once" logic.

## 🤖 Phase 4: Android & Termux Strategy
- [ ] **Termux Compatibility:** Ensure `cli-chat-rs` compiles and runs in Termux (ARM64).
- [ ] **Touch Support:** Add mouse/touch event handling to the TUI for mobile usage.
- [ ] **Web Server Mode:** Implement a mode where the backend serves the React App over `localhost` for Chrome Android access.

## 📚 Phase 5: Library Evolution (Generic)
*Note: These libraries are developed independently in `libs/` but drive WhaSwapp.*
- [ ] **`cli-chat-rs`:**
    - [ ] Add plugin system for protocol adapters.
    - [ ] Improve TUI widgets (Lists, Inputs).
- [ ] **`web-intelligence`:**
    - [ ] Improve browser detection logic.
    - [ ] Abstract the "AI Window" interface for general use.

## 🚀 Phase 6: Release Prep
- [ ] **Security Audit:** Scan for hardcoded secrets or leftover session files.
- [ ] **Performance Check:** Verify RAM usage meets the "Lightweight" goal.
- [ ] **Public Repo:** Finalize the "Clean Slate" commit and push to the new public repository.
