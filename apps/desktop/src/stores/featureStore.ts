import { create } from 'zustand';
// import { invoke } from '@tauri-apps/api/core';

interface FeatureState {
  capabilities: any;
  fetchCapabilities: () => Promise<void>;
}

export const useFeatureStore = create<FeatureState>((set) => ({
  capabilities: {},
  fetchCapabilities: async () => {
    try {
      // Stub for backend features, assume implemented
      // const caps = await invoke('get_backend_features');
      // set({ capabilities: caps });
      set({}); // avoiding unused variable error for now
    } catch (e) {
      console.error('Failed to fetch capabilities', e);
    }
  },
}));
