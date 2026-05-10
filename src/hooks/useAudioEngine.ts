import { useEffect } from 'react';
import { audioService, TranscriptPayload } from '../services/audioService';
import { useTranscriptStore } from '../store/useTranscriptStore';

/**
 * Hook that manages the audio engine lifecycle.
 * Robust against React 18's double-mount in dev mode.
 */
export function useAudioEngine(modelPath: string) {
  const addTranscript = useTranscriptStore((s) => s.addTranscript);
  const setEngineRunning = useTranscriptStore((s) => s.setEngineRunning);
  const setEngineLoading = useTranscriptStore((s) => s.setEngineLoading);
  const setError = useTranscriptStore((s) => s.setError);

  useEffect(() => {
    let isMounted = true;
    let micUnsub: (() => void) | null = null;
    let sysUnsub: (() => void) | null = null;

    const init = async () => {
      try {
        setEngineLoading(true);
        setError(null);

        // This call is now a singleton inside audioService
        await audioService.startEngine(modelPath);
        
        if (!isMounted) return;

        setEngineRunning(true);

        micUnsub = await audioService.onMicTranscript((payload: TranscriptPayload) => {
          if (isMounted) addTranscript(payload);
        });

        sysUnsub = await audioService.onSystemTranscript((payload: TranscriptPayload) => {
          if (isMounted) addTranscript(payload);
        });
      } catch (err) {
        if (isMounted) {
          console.error('[Akela] Engine start failed:', err);
          setError(String(err));
        }
      } finally {
        if (isMounted) {
          setEngineLoading(false);
        }
      }
    };

    init();

    return () => {
      isMounted = false;
      micUnsub?.();
      sysUnsub?.();
      
      // We DO NOT call audioService.stopEngine() here anymore.
      // This allows the engine to persist across hot-reloads and React 18 double-mounts.
      // The engine will naturally shut down when the Rust backend is dropped 
      // or if we explicitly call stopEngine elsewhere.
    };
  }, [modelPath, addTranscript, setEngineRunning, setEngineLoading, setError]);
}
