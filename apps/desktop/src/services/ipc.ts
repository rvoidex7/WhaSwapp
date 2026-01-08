// import { invoke } from '@tauri-apps/api/core';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useAuthStore } from '../stores/authStore';
import { useChatStore } from '../stores/chatStore';
import { useTerminalStore } from '../stores/terminalStore';

// Helper function to get timestamp with milliseconds
const getTimestamp = () => {
  const now = new Date();
  return `${now.toLocaleTimeString('en-US', { hour12: false })}.${now.getMilliseconds().toString().padStart(3, '0')}`;
};

// Override console methods to capture logs
const originalLog = console.log;
const originalError = console.error;
const originalWarn = console.warn;
const originalInfo = console.info;

console.log = (...args: any[]) => {
  originalLog(...args);
  useTerminalStore.getState().addLog({
    timestamp: getTimestamp(),
    level: 'INFO',
    source: 'FRONTEND',
    message: args.map(a => String(a)).join(' '),
  });
};

console.error = (...args: any[]) => {
  originalError(...args);
  useTerminalStore.getState().addLog({
    timestamp: getTimestamp(),
    level: 'ERROR',
    source: 'FRONTEND',
    message: args.map(a => String(a)).join(' '),
  });
};

console.warn = (...args: any[]) => {
  originalWarn(...args);
  useTerminalStore.getState().addLog({
    timestamp: getTimestamp(),
    level: 'WARN',
    source: 'FRONTEND',
    message: args.map(a => String(a)).join(' '),
  });
};

console.info = (...args: any[]) => {
  originalInfo(...args);
  useTerminalStore.getState().addLog({
    timestamp: getTimestamp(),
    level: 'INFO',
    source: 'FRONTEND',
    message: args.map(a => String(a)).join(' '),
  });
};

export const initIPC = async () => {
  console.log('Initializing Native IPC...');

  // Listen for generic backend events from our sidecar provider
  await listen('backend-event', (event: any) => {
    const payload = event.payload;
    // console.log('Backend Event:', payload); // Verbose

    if (payload.type === 'qr') {
        useAuthStore.getState().setQR(payload.payload);
    }
    else if (payload.type === 'connection_status') {
        const { status } = payload.payload;
        if (status === 'connected') {
             useAuthStore.getState().setConnected({ id: 'me', name: 'User' });
        } else if (status === 'disconnected') {
             useAuthStore.getState().setStatus('disconnected');
        } else if (status === 'logged_out') {
             useAuthStore.getState().setStatus('disconnected');
        }
    }
    else if (payload.type === 'message') {
        const msg = payload.payload;
        console.log('Message received:', msg);
        // Normalize message for store
        const normalized = {
            key: msg.key,
            fromMe: msg.key.fromMe,
            content: msg.message,
            timestamp: msg.messageTimestamp,
        };
        useChatStore.getState().addMessage(normalized);
    }
  });
};

export const sendText = async (jid: string, text: string) => {
    try {
        await invoke('send_message', { jid, content: text });
        // Optimistically add to store
        useChatStore.getState().addMessage({
            key: { fromMe: true, remoteJid: jid, id: 'temp-' + Date.now() },
            fromMe: true,
            content: { conversation: text },
            timestamp: Date.now() / 1000
        });
    } catch (e) {
        console.error("Failed to send message", e);
    }
};

// Stubs for missing functions to satisfy restored components
export const updateSettings = async (settings: any) => {
    console.log('Settings updated (stub):', settings);
    // return invoke('update_settings', { settings });
};

export const generateDraft = async (jid: string) => {
    console.log('Generating draft (stub) for:', jid);
    return "This is a generated draft response.";
    // return invoke('generate_draft', { jid });
};

export const markRead = async (_jid: string, _ids: string[]) => {
    // console.log('Mark read stub');
};

export const getMessages = async (_jid: string, _limit: number) => {
    return [];
};
