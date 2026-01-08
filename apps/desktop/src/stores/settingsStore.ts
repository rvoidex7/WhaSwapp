import { create } from 'zustand';

interface SettingsState {
  readReceipts: boolean;
  ghostMode: boolean;
  hideOnlineStatus: boolean;
  hideTyping: boolean;

  // LLM Settings
  llmEnabled: boolean;
  llmAutoSend: boolean;
  llmProvider: 'ollama' | 'lm-studio' | 'openai-compatible';
  llmApiUrl: string;
  llmModel: string;
  llmSystemPrompt: string;
  llmContextLimit: number;

  setReadReceipts: (enabled: boolean) => void;
  setGhostMode: (enabled: boolean) => void;
  setHideOnlineStatus: (enabled: boolean) => void;
  setHideTyping: (enabled: boolean) => void;

  setLlmEnabled: (enabled: boolean) => void;
  setLlmAutoSend: (enabled: boolean) => void;
  setLlmProvider: (provider: 'ollama' | 'lm-studio' | 'openai-compatible') => void;
  setLlmApiUrl: (url: string) => void;
  setLlmModel: (model: string) => void;
  setLlmSystemPrompt: (prompt: string) => void;
  setLlmContextLimit: (limit: number) => void;

  setAll: (settings: Partial<SettingsState>) => void;
}

export const useSettingsStore = create<SettingsState>((set) => ({
  readReceipts: true,
  ghostMode: false,
  hideOnlineStatus: false,
  hideTyping: false,

  llmEnabled: false,
  llmAutoSend: false,
  llmProvider: 'ollama',
  llmApiUrl: 'http://localhost:11434/v1',
  llmModel: 'llama3',
  llmSystemPrompt: 'You are a helpful assistant using WhatsApp. Keep your answers concise and natural.',
  llmContextLimit: 10,

  setReadReceipts: (enabled) => set({ readReceipts: enabled }),
  setGhostMode: (enabled) => set({ ghostMode: enabled }),
  setHideOnlineStatus: (enabled) => set({ hideOnlineStatus: enabled }),
  setHideTyping: (enabled) => set({ hideTyping: enabled }),

  setLlmEnabled: (enabled) => set({ llmEnabled: enabled }),
  setLlmAutoSend: (enabled) => set({ llmAutoSend: enabled }),
  setLlmProvider: (provider) => set({ llmProvider: provider }),
  setLlmApiUrl: (url) => set({ llmApiUrl: url }),
  setLlmModel: (model) => set({ llmModel: model }),
  setLlmSystemPrompt: (prompt) => set({ llmSystemPrompt: prompt }),
  setLlmContextLimit: (limit) => set({ llmContextLimit: limit }),

  setAll: (settings) => set((state) => ({ ...state, ...settings })),
}));
