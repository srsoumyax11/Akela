//! Audio Engine — Orchestrates dual-pipeline capture and transcription.
//!
//! Architecture (mirrors RealtimeSTT's 4-worker design):
//!   Mic ──► mic_capture ──► vad_buffer ──► stt_worker ──► Tauri Events
//!   Sys ──► system_capture ──► vad_buffer ──► stt_worker ──► Tauri Events

pub mod mic_capture;
pub mod system_capture;
pub mod vad_buffer;
pub mod stt_worker;
pub mod engine;
// pub mod silero_vad;

use serde::{Deserialize, Serialize};

// ── Shared Constants ──
// All values sourced from RealtimeSTT defaults
// (see realtimestt_deep_analysis.md §11)
pub const SAMPLE_RATE: u32 = 16_000;
pub const VAD_CHUNK_SAMPLES: usize = 512; // 32ms at 16kHz
pub const PRE_ROLL_SECONDS: f32 = 1.0;
pub const POST_SPEECH_SILENCE_MS: u64 = 600;
pub const MIN_RECORDING_MS: u64 = 500;
pub const SILERO_THRESHOLD: f32 = 0.6; // 1.0 - 0.4 sensitivity
pub const WEBRTC_MODE: i32 = 3; // Most aggressive
pub const MAX_QUEUED_CHUNKS: usize = 100;

// ── Shared Types ──

/// Identifies which pipeline produced a piece of audio or text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioSource {
    /// Your microphone
    Mic,
    /// System/loopback audio (meeting, video, etc.)
    System,
}

/// A timestamped chunk of 16kHz mono f32 audio, ready for VAD.
#[derive(Debug, Clone)]
pub struct AudioChunk {
    /// 512 f32 samples at 16kHz = 32ms of audio
    pub samples: Vec<f32>,
    /// Wall-clock timestamp when this chunk was captured (ms since epoch)
    pub timestamp_ms: u64,
    /// Which pipeline produced this chunk
    pub source: AudioSource,
}

/// Final transcript emitted to the frontend via Tauri events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptPayload {
    pub source: AudioSource,
    pub text: String,
    pub timestamp_ms: u64,
}
