import { create } from 'zustand';

interface ChatState {
  chats: any[];
  activeChat: string | null;
  messages: any[];
  drafts: Record<string, string>;
  setDraft: (jid: string, text: string) => void;
  setActiveChat: (jid: string) => void;
  addMessage: (msg: any) => void;
}

export const useChatStore = create<ChatState>((set) => ({
  chats: [],
  activeChat: null,
  messages: [],
  drafts: {},
  setDraft: (jid, text) => set((state) => ({ drafts: { ...state.drafts, [jid]: text } })),
  setActiveChat: (jid) => set({ activeChat: jid }),
  addMessage: (msg) => set((state) => ({ messages: [...state.messages, msg] })),
}));
