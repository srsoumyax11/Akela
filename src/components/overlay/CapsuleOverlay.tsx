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
import { invoke } from '@tauri-apps/api/core';
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
    console.log('Hiding window via Rust...');
    await invoke('hide_window');
  };

  const handleMouseDown = async (e: React.MouseEvent) => {
    // Only drag if the primary mouse button is pressed and we're not clicking a button or its children
    if (e.button === 0 && !(e.target as HTMLElement).closest('button')) {
      await appWindow.startDragging();
    }
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
