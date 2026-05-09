import { create } from 'zustand';

type PanelType = 'none' | 'chat' | 'history' | 'expanded' | 'settings';

interface OverlayState {
  // UI State
  activePanel: PanelType;
  isExpanded: boolean;
  
  // Audio State
  micEnabled: boolean;
  speakerEnabled: boolean;
  
  // Transcription
  currentTranscript: string;
  
  // Actions
  setActivePanel: (panel: PanelType) => void;
  toggleMic: () => void;
  toggleSpeaker: () => void;
  setTranscript: (text: string) => void;
}

export const useOverlayStore = create<OverlayState>((set) => ({
  activePanel: 'none',
  isExpanded: false,
  micEnabled: true,
  speakerEnabled: true,
  currentTranscript: '',

  setActivePanel: (panel) => set({ activePanel: panel }),
  toggleMic: () => set((state) => ({ micEnabled: !state.micEnabled })),
  toggleSpeaker: () => set((state) => ({ speakerEnabled: !state.speakerEnabled })),
  setTranscript: (text) => set({ currentTranscript: text }),
}));
