# WhaSwapp Features & Capabilities

This document outlines the feature set of WhaSwapp, categorizing them into Core (Standard), Extras (Our Additions), and Forbidden (Ban Risk).

## ✅ Core Features (Standard)
*Inherited from the underlying libraries (Baileys/Rust).*

*   **Messaging:** Send/Receive text, images, videos, audio.
*   **Authentication:** QR Code login via Multi-Device.
*   **Security:** End-to-End Encryption (handled by the libraries).
*   **Notifications:** Desktop notifications for new messages.

## 🚀 WhaSwapp Extras (Safe & Client-Side)
*These features are implemented on the client-side (React/Rust) without violating protocol limits.*

### 👻 Ghost Mode (Privacy)
*   **Hide Blue Ticks:** Read messages without sending the "Read" receipt.
*   **Hide Typing Status:** Type messages without sending the "Typing..." indicator.
*   **Hide Online Status:** (Experimental) Attempt to keep "Last Seen" frozen or hidden.

### 🛡️ Data Retention (Anti-Delete)
*   **Anti-Delete Messages:** When a contact uses "Delete for Everyone", WhaSwapp ignores the delete command and keeps the message in the UI, marked with a small icon/indicator.
*   **Anti-Delete Status:** View statuses even after they have been deleted by the owner.
*   **Save View Once:** Prevent "View Once" media from disappearing; allow saving to disk.

### 🧠 Local Intelligence (AI)
*   **Local LLM Integration:** Use `web-intelligence` to leverage browser-based Local LLMs (e.g., Gemini Nano).
*   **Summarization:** Summarize long group chats locally.
*   **Smart Replies:** Generate draft replies based on context.
*   *Note: No chat data is sent to external cloud AI servers.*

### 🛠️ UI & UX Customizations
*   **Themes:** Full Dark/Light mode and custom color schemes (React-based).
*   **Hide/Lock Chats:** Hide specific chats from the list or lock them with a password (Client-side logic).
*   **Disable Forwarded Tag:** Forward a message by copying its content to a new message, effectively removing the "Forwarded" label.
*   **DND Mode:** "Do Not Disturb" toggle to suspend network traffic or notifications for WhaSwapp only.
*   **Download Status:** Button to save photos/videos from contacts' statuses.

## ⛔ Forbidden Features (Red Zone)
*These features are strictly prohibited due to high risk of account bans.*

*   ❌ **Bulk Messaging / Auto-Spam:** Automated mass messaging tools.
*   ❌ **Media Limit Bypass:** Sending files larger than official limits (e.g., 1GB video).
*   ❌ **Viral Limit Bypass:** Forwarding messages to more than 5 chats at once.
*   ❌ **Status Duration Bypass:** Uploading statuses longer than 30 seconds (if validated by server).

> **Philosophy:** We enhance the *user's control* over their own data, but we do not abuse the *WhatsApp network* or spam other users.
