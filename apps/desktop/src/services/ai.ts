// Service to interact with Chrome's built-in AI (window.ai)

// Type definitions for the emerging standard
interface AILanguageModel {
  create(options?: any): Promise<AILanguageModelSession>;
  availability(): Promise<"available" | "downloadable" | "unavailable">;
  capabilities(): Promise<any>;
}

interface AILanguageModelSession {
  prompt(input: string): Promise<string>;
  promptStreaming(input: string): ReadableStream<string>;
  destroy(): void;
}

interface WindowAI {
  languageModel: AILanguageModel;
}

// Extend global window type
declare global {
  interface Window {
    ai?: WindowAI;
    // Fallbacks for older implementations/demos
    model?: any;
  }
}

export const aiService = {
  isAvailable: async (): Promise<boolean> => {
    if (!window.ai || !window.ai.languageModel) {
      console.warn("window.ai or window.ai.languageModel not found");
      return false;
    }
    try {
      const status = await window.ai.languageModel.availability();
      console.log("AI Model Status:", status);
      return status === "available" || status === "downloadable"; // Downloadable counts as potentially available
    } catch (e) {
      console.error("Error checking AI availability:", e);
      return false;
    }
  },

  createSession: async (): Promise<AILanguageModelSession | null> => {
    if (!window.ai?.languageModel) return null;
    try {
      return await window.ai.languageModel.create();
    } catch (e) {
      console.error("Failed to create AI session:", e);
      return null;
    }
  },

  generateText: async (prompt: string): Promise<string> => {
    const session = await aiService.createSession();
    if (!session) throw new Error("AI Session creation failed");

    try {
      const result = await session.prompt(prompt);
      session.destroy();
      return result;
    } catch (e) {
      session.destroy();
      throw e;
    }
  }
};
