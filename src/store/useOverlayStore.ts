import { create } from 'zustand';
import { audioService } from '../services/audioService';

type PanelType = 'none' | 'chat' | 'history' | 'expanded' | 'settings';

interface OverlayState {
  activePanel: PanelType;
  isExpanded: boolean;
  micEnabled: boolean;
  speakerEnabled: boolean;
  
  setActivePanel: (panel: PanelType) => void;
  toggleMic: () => void;
  toggleSpeaker: () => void;
}

export const useOverlayStore = create<OverlayState>((set) => ({
  activePanel: 'none',
  isExpanded: false,
  micEnabled: true,
  speakerEnabled: true,

  setActivePanel: (panel) => set({ activePanel: panel }),
  
  toggleMic: () => set((state) => {
    const next = !state.micEnabled;
    audioService.setMicEnabled(next).catch(console.error);
    return { micEnabled: next };
  }),

  toggleSpeaker: () => set((state) => {
    const next = !state.speakerEnabled;
    audioService.setSpeakerEnabled(next).catch(console.error);
    return { speakerEnabled: next };
  }),
}));
