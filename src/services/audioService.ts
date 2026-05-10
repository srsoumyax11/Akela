import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export interface TranscriptPayload {
  source: 'mic' | 'system';
  text: string;
  timestamp_ms: u64;
}

type u64 = number;

/**
 * Singleton state to prevent double-init issues in React 18 Dev mode
 */
let isStarting = false;
let isRunning = false;

export const audioService = {
  async startEngine(modelPath: string): Promise<void> {
    if (isRunning) return;
    if (isStarting) {
      // Wait for the other start to finish
      while (isStarting) {
        await new Promise(r => setTimeout(r, 100));
      }
      return;
    }

    try {
      isStarting = true;
      await invoke('start_audio_engine', { modelPath });
      isRunning = true;
    } catch (err) {
      // If the error says "already running", we consider it a success
      if (typeof err === 'string' && err.includes('already running')) {
        isRunning = true;
        return;
      }
      throw err;
    } finally {
      isStarting = false;
    }
  },

  async stopEngine(): Promise<void> {
    // In React 18 Dev mode, we avoid stopping immediately on unmount 
    // to prevent killing the second mount's instance.
    // We only stop if we are truly shutting down.
    isRunning = false;
    return invoke('stop_audio_engine');
  },

  async setMicEnabled(enabled: boolean): Promise<void> {
    return invoke('set_mic_enabled', { enabled });
  },

  async setSpeakerEnabled(enabled: boolean): Promise<void> {
    return invoke('set_speaker_enabled', { enabled });
  },

  async onMicTranscript(callback: (payload: TranscriptPayload) => void) {
    return listen<TranscriptPayload>('transcript:mic', (event) => {
      callback(event.payload);
    });
  },

  async onSystemTranscript(callback: (payload: TranscriptPayload) => void) {
    return listen<TranscriptPayload>('transcript:system', (event) => {
      callback(event.payload);
    });
  },
};
