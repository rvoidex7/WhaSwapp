import { create } from 'zustand';

interface Log {
  timestamp: string;
  level: string;
  source: string;
  message: string;
}

interface TerminalState {
  logs: Log[];
  addLog: (log: Log) => void;
  clearLogs: () => void;
}

export const useTerminalStore = create<TerminalState>((set) => ({
  logs: [],
  addLog: (log) => set((state) => ({ logs: [...state.logs, log].slice(-1000) })), // Keep last 1000
  clearLogs: () => set({ logs: [] }),
}));
