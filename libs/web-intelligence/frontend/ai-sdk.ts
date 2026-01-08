// Type definitions for the Chrome Built-in AI API
// Supports both legacy (text session) and new (language model) APIs

// --- New API Spec (Prompt API) ---
export interface AILanguageModel {
    prompt(text: string): Promise<string>;
    promptStreaming(text: string): ReadableStream<string>;
    destroy(): void;
    clone(): Promise<AILanguageModel>;
    countPromptTokens(text: string): Promise<number>;
    maxTokens: number;
    tokensLeft: number;
    topK: number;
    temperature: number;
}

export interface AILanguageModelCreateOptions {
    topK?: number;
    temperature?: number;
    systemPrompt?: string;
}

export interface AIAvailabilityOptions {
    topK?: number;
    temperature?: number;
}

export type AICapabilityAvailability = 'readily' | 'after-download' | 'no';

export interface AILanguageModelFactory {
    create(options?: AILanguageModelCreateOptions): Promise<AILanguageModel>;
    capabilities(): Promise<AILanguageModelCapabilities>;
}

export interface AILanguageModelCapabilities {
    available: AICapabilityAvailability;
    defaultTopK: number;
    maxTopK: number;
    defaultTemperature: number;
}


// --- Legacy API Spec (Text Session) ---
export interface AIModelLegacy {
    prompt(text: string): Promise<string>;
    promptStreaming(text: string): ReadableStream<string>;
    destroy(): void;
}

export interface AITextSessionOptions {
    topK?: number;
    temperature?: number;
}

export interface WindowAILegacy {
    canCreateTextSession(): Promise<AICapabilityAvailability>;
    createTextSession(options?: AITextSessionOptions): Promise<AIModelLegacy>;
}


// --- Global Declaration ---
declare global {
    interface Window {
        ai?: {
            languageModel?: AILanguageModelFactory;
        } & Partial<WindowAILegacy>;
    }
}

/**
 * Helper to check availability and create a session.
 * Abstractions over both new and legacy APIs.
 */
export class AIClient {
    private model: AILanguageModel | AIModelLegacy | null = null;

    /**
     * Checks if AI is available (either via new languageModel or legacy textSession).
     */
    async isAvailable(): Promise<boolean> {
        if (typeof window.ai === 'undefined') {
            return false;
        }

        // Check new API
        if (window.ai.languageModel) {
            try {
                const caps = await window.ai.languageModel.capabilities();
                return caps.available === 'readily';
            } catch (e) {
                console.warn("Error checking languageModel capabilities:", e);
            }
        }

        // Check legacy API
        if (window.ai.canCreateTextSession) {
            try {
                const status = await window.ai.canCreateTextSession();
                return status === 'readily';
            } catch (e) {
                console.warn("Error checking legacy canCreateTextSession:", e);
            }
        }

        return false;
    }

    /**
     * Creates an AI session.
     */
    async createSession(options?: AILanguageModelCreateOptions): Promise<AILanguageModel | AIModelLegacy> {
        if (!window.ai) {
            throw new Error("AI API not available in this browser");
        }

        // Try new API
        if (window.ai.languageModel) {
            try {
                this.model = await window.ai.languageModel.create(options);
                return this.model;
            } catch (e) {
                console.warn("Failed to create languageModel, falling back to legacy...", e);
            }
        }

        // Try legacy API
        if (window.ai.createTextSession) {
             // Map new options to legacy options if needed
            this.model = await window.ai.createTextSession(options as AITextSessionOptions);
            return this.model;
        }

        throw new Error("No compatible AI API found (neither languageModel nor createTextSession worked)");
    }

    async generate(prompt: string): Promise<string> {
        if (!this.model) {
            throw new Error("Session not created. Call createSession() first.");
        }
        return this.model.prompt(prompt);
    }

    destroy() {
        if (this.model) {
            this.model.destroy();
            this.model = null;
        }
    }
}
