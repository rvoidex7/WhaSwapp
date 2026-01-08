import { create } from 'zustand';

interface AuthState {
  status: 'disconnected' | 'connecting' | 'qr' | 'connected';
  qrCode: string | null;
  user: any | null;
  setStatus: (status: AuthState['status']) => void;
  setQR: (qr: string) => void;
  setConnected: (user: any) => void;
  reset: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  status: 'disconnected',
  qrCode: null,
  user: null,
  setStatus: (status) => set({ status }),
  setQR: (qr) => set({ status: 'qr', qrCode: qr }),
  setConnected: (user) => set({ status: 'connected', user, qrCode: null }),
  reset: () => set({ status: 'disconnected', qrCode: null, user: null }),
}));
