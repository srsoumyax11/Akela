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
} from 'lucide-react';
import { useFocusSafeDrag } from '../../hooks/useFocusSafeDrag';
import { windowService } from '../../services/windowService';
import { useOverlayStore } from '../../store/useOverlayStore';
import './CapsuleOverlay.css';

export const CapsuleOverlay: React.FC = () => {
  const {
    micEnabled,
    speakerEnabled,
    currentTranscript,
    toggleMic,
    toggleSpeaker,
    setActivePanel
  } = useOverlayStore();

  const { handleMouseDown } = useFocusSafeDrag();

  const handleClose = () => {
    windowService.hide();
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
        <div className="ticker-text">
          {currentTranscript || 'Waiting for audio context... listening in background'}
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
    </div>
  );
};
