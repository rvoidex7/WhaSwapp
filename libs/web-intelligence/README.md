# web-intelligence

A Rust library to launch and configure web browsers (Chrome, Edge, Chromium) with "Built-in AI" features enabled for development purposes.

This crate helps developers create a consistent, isolated environment for testing web applications that utilize local AI models (like Gemini Nano) via the `window.ai` API.

## Features

- **Automatic Browser Detection:** Finds installations of Chrome, Chrome Canary/Dev, Chromium, and Edge on Windows, macOS, and Linux.
- **Isolated Profiles:** Creates separate user data directories for development to avoid polluting your main browser profile.
- **AI-Ready Configuration:** Automatically sets the necessary experimental flags (e.g., `OptimizationGuideModelDownloading`, `PromptAPIForGeminiNano`) to enable on-device AI features.
- **Frontend SDK:** Includes a TypeScript helper (`frontend/ai-sdk.ts`) to bridge the gap between legacy and modern `window.ai` APIs.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
web-intelligence = { path = "./web-intelligence" }
# Or from crates.io once published
# web-intelligence = "0.1.0"
```

## Usage (Rust)

```rust
use web_intelligence::BrowserLauncher;

fn main() {
    // Create a launcher with a specific profile name (stored in system cache)
    let launcher = BrowserLauncher::new("my-ai-dev-profile");

    // Launch in AI Development mode
    // - Opens the browser pointing to your local app
    // - Enables Remote Debugging on port 9222
    // - Enables Gemini Nano and other AI flags
    match launcher.launch_for_ai_dev("http://localhost:3000", 9222) {
        Ok(child) => println!("Browser launched with PID: {}", child.id()),
        Err(e) => eprintln!("Error launching browser: {}", e),
    }
}
```

## Usage (Frontend)

The library provides a TypeScript SDK to simplify working with the experimental AI APIs. Copy `frontend/ai-sdk.ts` to your project.

```typescript
import { AIClient } from './ai-sdk';

const ai = new AIClient();

async function runAI() {
  if (await ai.isAvailable()) {
    const session = await ai.createSession({
      temperature: 0.7,
      topK: 5
    });

    const stream = session.promptStreaming("Explain quantum computing simply.");
    for await (const chunk of stream) {
      console.log(chunk);
    }
  }
}
```

## Related Resources

- [Chrome Built-in AI Documentation](https://developer.chrome.com/docs/ai/built-in)
- [Explainer: Prompt API for Gemini Nano](https://github.com/explainers-by-googlers/prompt-api)
- [Web Machine Learning](https://webmachinelearning.github.io/)
