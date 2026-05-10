import React from 'react';
import {
  Sparkles,
  Mic,
  MicOff,
  Volume2,
  VolumeX,
  Settings,
  X,
  PawPrint,
  Loader2,
} from 'lucide-react';
import { useFocusSafeDrag } from '../../hooks/useFocusSafeDrag';
import { useAudioEngine } from '../../hooks/useAudioEngine';
import { windowService } from '../../services/windowService';
import { useOverlayStore } from '../../store/useOverlayStore';
import { useTranscriptStore } from '../../store/useTranscriptStore';
import './CapsuleOverlay.css';

// Model path relative to the executable / project root
// In dev mode, Tauri runs from src-tauri, so we go up one level
const MODEL_PATH = '../models/ggml-base.en.bin';

export const CapsuleOverlay: React.FC = () => {
  const {
    micEnabled,
    speakerEnabled,
    toggleMic,
    toggleSpeaker,
    setActivePanel
  } = useOverlayStore();

  const { liveText, engineLoading, engineRunning, error } = useTranscriptStore();

  // Initialize the audio engine
  useAudioEngine(MODEL_PATH);

  const { handleMouseDown } = useFocusSafeDrag();

  const handleClose = () => {
    windowService.hide();
  };

  // Determine ticker content based on engine state
  const getTickerContent = () => {
    if (error) return `⚠ Engine error: ${error}`;
    if (engineLoading) return '🔄 Loading Whisper model...';
    if (!engineRunning) return 'Engine offline';
    return liveText;
  };

  const getTickerClass = () => {
    if (error) return 'ticker-text ticker-error';
    if (engineLoading) return 'ticker-text ticker-loading';
    if (liveText !== 'Listening...') return 'ticker-text ticker-active';
    return 'ticker-text';
  };

  return (
    <div className="capsule-overlay">
      <div
        className="capsule-logo"
        onMouseDown={handleMouseDown}
        style={{ cursor: 'move' }}
      >
        <PawPrint className="logo-icon" strokeWidth={2.5} />
      </div>

      <div className="ticker-container">
        <div className={getTickerClass()}>
          {engineLoading && <Loader2 size={12} className="ticker-spinner" />}
          {getTickerContent()}
        </div>
      </div>

      <div className="capsule-actions">
        <button className="btn-help" onClick={() => setActivePanel('chat')}>
          <Sparkles size={14} />
          Help Me
        </button>

        <button
          className={`icon-btn ${micEnabled ? 'active' : 'inactive'}`}
          onClick={toggleMic}
          title={micEnabled ? 'Mute Microphone' : 'Unmute Microphone'}
        >
          {micEnabled ? <Mic size={16} /> : <MicOff size={16} />}
        </button>

        <button
          className={`icon-btn ${speakerEnabled ? 'active' : 'inactive'}`}
          onClick={toggleSpeaker}
          title={speakerEnabled ? 'Mute System Audio' : 'Unmute System Audio'}
        >
          {speakerEnabled ? <Volume2 size={16} /> : <VolumeX size={16} />}
        </button>

        <button
          className="icon-btn"
          onClick={() => setActivePanel('settings')}
          title="Settings"
        >
          <Settings size={16} />
        </button>

        <button
          className="icon-btn close-btn"
          onClick={handleClose}
          title="Close Akela"
        >
          <X size={16} />
        </button>
      </div>

      {/* Engine status indicator */}
      <div className={`engine-dot ${engineRunning ? 'running' : engineLoading ? 'loading' : 'offline'}`} />
    </div>
  );
};
