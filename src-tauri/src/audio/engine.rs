//! Audio Engine — Orchestrator that wires up the complete dual-pipeline.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::Result;
use crossbeam_channel::{bounded, Sender};
use tauri::{AppHandle, Emitter};
use tracing::{error, info, warn, debug};

use super::mic_capture;
use super::stt_worker::{self, WhisperEngine};
use super::system_capture;
use super::vad_buffer;
use super::{AudioSource, TranscriptPayload};

pub struct AudioEngine {
    _handles: Vec<std::thread::JoinHandle<()>>,
    mic_enabled: Arc<AtomicBool>,
    system_enabled: Arc<AtomicBool>,
    /// Global shutdown flag to stop all threads
    shutdown: Arc<AtomicBool>,
}

impl AudioEngine {
    pub fn start(app_handle: AppHandle, model_path: PathBuf) -> Result<Self> {
        info!("Starting Audio Engine...");

        let engine = WhisperEngine::new(&model_path)?;
        engine.warm_up()?;
        let whisper_ctx = engine.context();

        let mut handles = Vec::new();
        let mic_enabled = Arc::new(AtomicBool::new(true));
        let system_enabled = Arc::new(AtomicBool::new(true));
        let shutdown = Arc::new(AtomicBool::new(false));

        // ── Mic Pipeline ──
        let (mic_chunk_tx, mic_chunk_rx) = bounded(100);
        let (mic_record_tx, mic_record_rx) = bounded(10);
        let (mic_transcript_tx, mic_transcript_rx) = bounded(10);

        // Capture
        handles.push(mic_capture::start_mic_capture(
            mic_chunk_tx,
            Arc::clone(&shutdown)
        )?);
        
        // VAD
        handles.push(vad_buffer::start_vad_pipeline(
            AudioSource::Mic,
            Arc::clone(&mic_enabled),
            Arc::clone(&shutdown),
            mic_chunk_rx,
            mic_record_tx,
        ));
        
        // STT
        handles.push(stt_worker::start_stt_worker(
            AudioSource::Mic,
            Arc::clone(&whisper_ctx),
            mic_record_rx,
            mic_transcript_tx,
        ));

        // Emitter
        let app = app_handle.clone();
        let s_mic = Arc::clone(&shutdown);
        handles.push(std::thread::spawn(move || {
            emit_transcripts(app, mic_transcript_rx, "transcript:mic", s_mic)
        }));

        // ── System Pipeline ──
        let (sys_chunk_tx, sys_chunk_rx) = bounded(100);
        let (sys_record_tx, sys_record_rx) = bounded(10);
        let (sys_transcript_tx, sys_transcript_rx) = bounded(10);

        // Capture
        handles.push(system_capture::start_system_capture(
            sys_chunk_tx,
            Arc::clone(&shutdown)
        )?);
        
        // VAD
        handles.push(vad_buffer::start_vad_pipeline(
            AudioSource::System,
            Arc::clone(&system_enabled),
            Arc::clone(&shutdown),
            sys_chunk_rx,
            sys_record_tx,
        ));
        
        // STT
        handles.push(stt_worker::start_stt_worker(
            AudioSource::System,
            Arc::clone(&whisper_ctx),
            sys_record_rx,
            sys_transcript_tx,
        ));

        // Emitter
        let app = app_handle.clone();
        let s_sys = Arc::clone(&shutdown);
        handles.push(std::thread::spawn(move || {
            emit_transcripts(app, sys_transcript_rx, "transcript:system", s_sys)
        }));

        Ok(Self {
            _handles: handles,
            mic_enabled,
            system_enabled,
            shutdown,
        })
    }

    pub fn set_mic_enabled(&self, enabled: bool) {
        info!("AudioEngine: set_mic_enabled -> {}", enabled);
        self.mic_enabled.store(enabled, Ordering::Relaxed);
    }

    pub fn set_system_enabled(&self, enabled: bool) {
        info!("AudioEngine: set_system_enabled -> {}", enabled);
        self.system_enabled.store(enabled, Ordering::Relaxed);
    }
}

impl Drop for AudioEngine {
    fn drop(&mut self) {
        info!("AudioEngine: Dropping engine instance. Shutting down threads...");
        self.shutdown.store(true, Ordering::Relaxed);
    }
}

fn emit_transcripts(
    app: AppHandle,
    rx: crossbeam_channel::Receiver<TranscriptPayload>,
    event_name: &str,
    shutdown: Arc<AtomicBool>,
) {
    loop {
        if shutdown.load(Ordering::Relaxed) { break; }
        
        // Use recv_timeout to check shutdown periodically
        match rx.recv_timeout(std::time::Duration::from_millis(200)) {
            Ok(payload) => {
                if let Err(e) = app.emit(event_name, &payload) {
                    error!("Failed to emit {event_name}: {e}");
                }
            }
            Err(crossbeam_channel::RecvTimeoutError::Timeout) => continue,
            Err(crossbeam_channel::RecvTimeoutError::Disconnected) => break,
        }
    }
    debug!("Emitter thread for {event_name} exiting");
}
