import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

interface SettingsState {
  activeProvider: 'groq' | 'openai' | 'mistral';
  apiKeys: {
    groq: string;
    openai: string;
    mistral: string;
  };
  setActiveProvider: (provider: 'groq' | 'openai' | 'mistral') => void;
  setApiKey: (provider: 'groq' | 'openai' | 'mistral', key: string) => void;
  saveApiKeyToDb: (provider: 'groq' | 'openai' | 'mistral', key: string) => Promise<void>;
  loadApiKeys: () => Promise<void>;
}

// For now, storing in memory. SQLite integration will come later.
export const useSettingsStore = create<SettingsState>((set, get) => ({
  activeProvider: 'groq',
  apiKeys: {
    groq: '',
    openai: '',
    mistral: '',
  },
  setActiveProvider: (provider) => set({ activeProvider: provider }),
  setApiKey: (provider, key) => set((state) => ({
    apiKeys: { ...state.apiKeys, [provider]: key }
  })),
  saveApiKeyToDb: async (provider, key) => {
    try {
      await invoke('save_api_key', { provider, keyValue: key });
      console.log(`Saved API key for ${provider}`);
    } catch (err) {
      console.error(`Failed to save API key for ${provider}:`, err);
    }
  },
  loadApiKeys: async () => {
    try {
      const groq = await invoke<string | null>('get_api_key', { provider: 'groq' });
      const openai = await invoke<string | null>('get_api_key', { provider: 'openai' });
      const mistral = await invoke<string | null>('get_api_key', { provider: 'mistral' });
      
      set((state) => ({
        apiKeys: {
          groq: groq || state.apiKeys.groq,
          openai: openai || state.apiKeys.openai,
          mistral: mistral || state.apiKeys.mistral,
        }
      }));
    } catch (err) {
      console.error('Failed to load API keys:', err);
    }
  }
}));
