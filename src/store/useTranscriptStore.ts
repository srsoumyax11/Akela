import { create } from 'zustand';
import { TranscriptPayload } from '../services/audioService';

interface TranscriptEntry {
  id: string;
  source: 'mic' | 'system';
  text: string;
  timestamp: number;
}

interface TranscriptState {
  /** Whether the audio engine is currently running */
  engineRunning: boolean;
  /** Whether the engine is loading (model warmup) */
  engineLoading: boolean;
  /** Current live transcript text for the ticker */
  liveText: string;
  /** Full transcript history */
  entries: TranscriptEntry[];
  /** Error message if engine failed to start */
  error: string | null;

  setEngineRunning: (running: boolean) => void;
  setEngineLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  addTranscript: (payload: TranscriptPayload) => void;
  clearTranscripts: () => void;
}

export const useTranscriptStore = create<TranscriptState>((set) => ({
  engineRunning: false,
  engineLoading: false,
  liveText: 'Listening...',
  entries: [],
  error: null,

  setEngineRunning: (running) => set({ engineRunning: running }),
  setEngineLoading: (loading) => set({ engineLoading: loading }),
  setError: (error) => set({ error }),

  addTranscript: (payload) => {
    const entry: TranscriptEntry = {
      id: `${payload.source}-${payload.timestamp_ms}`,
      source: payload.source,
      text: payload.text,
      timestamp: payload.timestamp_ms,
    };

    set((state) => ({
      entries: [...state.entries, entry],
      liveText: payload.text,
    }));
  },

  clearTranscripts: () => set({ entries: [], liveText: 'Listening...' }),
}));
