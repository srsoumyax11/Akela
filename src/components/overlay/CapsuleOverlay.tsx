import React from 'react';
import { 
  Sparkles, 
  Mic, 
  MicOff, 
  Volume2, 
  VolumeX, 
  Settings, 
  X,
  Bird
} from 'lucide-react';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useOverlayStore } from '../../store/useOverlayStore';
import './CapsuleOverlay.css';

const appWindow = getCurrentWindow();

export const CapsuleOverlay: React.FC = () => {
  const { 
    micEnabled, 
    speakerEnabled, 
    currentTranscript, 
    toggleMic, 
    toggleSpeaker,
    setActivePanel 
  } = useOverlayStore();

  const handleClose = async () => {
    await appWindow.close();
  };

  const handleMouseDown = async (e: React.MouseEvent) => {
    // Only drag if the primary mouse button is pressed and we're not clicking a button
    if (e.button === 0 && !(e.target instanceof HTMLButtonElement)) {
      await appWindow.startDragging();
    }
  };

  return (
    <div 
      className="capsule-overlay" 
      data-tauri-drag-region 
      onMouseDown={handleMouseDown}
    >
      <div className="capsule-logo" data-tauri-drag-region>
        <Bird className="logo-icon" strokeWidth={2.5} data-tauri-drag-region />
      </div>

      <div className="ticker-container" data-tauri-drag-region>
        <div className="ticker-text" data-tauri-drag-region>
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
