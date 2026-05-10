//! STT Worker — Whisper Inference Engine
//!
//! Receives finalized audio from `vad_buffer`, runs Whisper inference,
//! and emits TranscriptPayload events to the Tauri frontend.
//!
//! Architecture (mirrors RealtimeSTT's transcript_process):
//!   - One `WhisperContext` loaded once (model weights, ~150MB for base.en)
//!   - Separate `WhisperState` per pipeline (Mic vs System)
//!   - Each worker runs on a dedicated thread
//!
//! Key optimizations from RealtimeSTT:
//!   - Model warm-up on startup (§6.2 in deep analysis)
//!   - Peak normalization to 0.95 before inference (§6.3)

use std::path::Path;
use std::sync::Arc;

use anyhow::{Context, Result};
use crossbeam_channel::{Receiver, Sender};
use tracing::{debug, error, info, warn};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

use super::vad_buffer::RecordingResult;
use super::{AudioSource, TranscriptPayload, SAMPLE_RATE};

/// Shared Whisper model context (thread-safe, read-only after init).
/// Both Mic and System pipelines share this to save memory.
pub struct WhisperEngine {
    ctx: Arc<WhisperContext>,
}

impl WhisperEngine {
    /// Load the Whisper model from disk.
    ///
    /// `model_path` should point to a GGML model file (e.g., `ggml-base.en.bin`).
    pub fn new(model_path: &Path) -> Result<Self> {
        info!("Loading Whisper model from: {}", model_path.display());

        let params = WhisperContextParameters::default();
        let ctx = WhisperContext::new_with_params(
            model_path.to_str().context("Invalid model path")?,
            params,
        )
        .map_err(|e| anyhow::anyhow!("Failed to load Whisper model: {e}"))?;

        info!("Whisper model loaded successfully");

        Ok(Self {
            ctx: Arc::new(ctx),
        })
    }

    /// Warm up the model by transcribing silence.
    /// This forces model weights into CPU cache, eliminating first-inference lag.
    /// (Mirrors: audio_recorder.py L804-811)
    pub fn warm_up(&self) -> Result<()> {
        info!("Warming up Whisper model...");

        let mut state = self
            .ctx
            .create_state()
            .map_err(|e| anyhow::anyhow!("Failed to create warmup state: {e}"))?;

        // 1 second of silence at 16kHz
        let silence = vec![0.0f32; SAMPLE_RATE as usize];
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some("en"));
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_suppress_blank(true);

        state
            .full(params, &silence)
            .map_err(|e| anyhow::anyhow!("Warmup inference failed: {e}"))?;

        info!("Whisper warm-up complete");
        Ok(())
    }

    /// Get a shared reference to the context for creating worker states.
    pub fn context(&self) -> Arc<WhisperContext> {
        Arc::clone(&self.ctx)
    }
}

/// Starts an STT worker thread for a specific pipeline.
///
/// Receives `RecordingResult` from the VAD pipeline, runs Whisper,
/// and sends `TranscriptPayload` back.
pub fn start_stt_worker(
    source: AudioSource,
    whisper_ctx: Arc<WhisperContext>,
    recording_rx: Receiver<RecordingResult>,
    transcript_tx: Sender<TranscriptPayload>,
) -> std::thread::JoinHandle<()> {
    let source_name = match source {
        AudioSource::Mic => "mic",
        AudioSource::System => "system",
    };

    std::thread::Builder::new()
        .name(format!("stt-{source_name}"))
        .spawn(move || {
            if let Err(e) =
                run_stt_loop(source, whisper_ctx, recording_rx, transcript_tx)
            {
                error!("STT worker ({source_name}) failed: {e:#}");
            }
        })
        .expect("Failed to spawn STT worker thread")
}

fn run_stt_loop(
    source: AudioSource,
    whisper_ctx: Arc<WhisperContext>,
    recording_rx: Receiver<RecordingResult>,
    transcript_tx: Sender<TranscriptPayload>,
) -> Result<()> {
    let source_name = match source {
        AudioSource::Mic => "mic",
        AudioSource::System => "system",
    };

    // Create a dedicated WhisperState for this pipeline
    let mut state = whisper_ctx
        .create_state()
        .map_err(|e| anyhow::anyhow!("Failed to create Whisper state: {e}"))?;

    info!("[{source_name}] STT worker ready");

    loop {
        let recording = match recording_rx.recv() {
            Ok(r) => r,
            Err(_) => {
                debug!("[{source_name}] Recording channel closed, exiting STT loop");
                break;
            }
        };

        let sample_count = recording.samples.len();
        let duration_s = sample_count as f32 / SAMPLE_RATE as f32;

        if sample_count == 0 {
            continue;
        }

        info!(
            "[{source_name}] Transcribing {:.1}s of audio ({} samples)...",
            duration_s, sample_count
        );

        // ── Peak Normalization ──
        // Normalize audio to 0.95 peak before inference.
        // This ensures consistent input levels regardless of mic gain.
        // (Mirrors: audio_recorder.py L183-188)
        let mut samples = recording.samples;
        let peak = samples.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
        if peak > 0.0 {
            let scale = 0.95 / peak;
            for s in samples.iter_mut() {
                *s *= scale;
            }
        }

        // ── Whisper Inference ──
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some("en"));
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_suppress_blank(true);
        params.set_n_threads(4);

        match state.full(params, &samples) {
            Ok(_) => {
                // Extract text from all segments using the 0.16 API:
                // `full_n_segments()` returns i32 directly
                // `get_segment(i)` returns Option<WhisperSegment>
                // WhisperSegment has `to_str_lossy()` which returns the text
                let num_segments = state.full_n_segments();

                let mut text = String::new();
                for i in 0..num_segments {
                    if let Some(seg) = state.get_segment(i) {
                        // WhisperSegment implements Display via to_str_lossy
                        let seg_text = format!("{seg}");
                        text.push_str(&seg_text);
                    }
                }

                let text = text.trim().to_string();

                if !text.is_empty() {
                    info!("[{source_name}] Transcript: \"{text}\"");

                    let payload = TranscriptPayload {
                        source,
                        text,
                        timestamp_ms: recording.timestamp_ms,
                    };

                    if transcript_tx.send(payload).is_err() {
                        debug!("[{source_name}] Transcript channel closed");
                        break;
                    }
                } else {
                    debug!("[{source_name}] Empty transcript (silence or noise)");
                }
            }
            Err(e) => {
                warn!("[{source_name}] Whisper inference failed: {e}");
            }
        }
    }

    info!("[{source_name}] STT worker stopped");
    Ok(())
}
