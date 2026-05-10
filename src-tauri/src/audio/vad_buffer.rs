//! VAD + Pre-Roll Buffer Pipeline
//!
//! This is the "brain" of each audio pipeline. It implements the three key
//! algorithms from RealtimeSTT (see realtimestt_deep_analysis.md §3-5):
//!
//! A. VAD Scout — WebRTC (fast scout) filters audio frames.
//!
//! B. Pre-Roll Ring Buffer — A circular buffer always holding the last ~1s
//!    of audio. When voice is detected, this buffered audio is prepended
//!    so the first syllable is never clipped.
//!    (Mirrors: audio_recorder.py L939-942, L2090-2098)
//!
//! C. Silence-Based Stop — After speech ends, we wait 600ms of continuous
//!    silence before finalizing. If speech resumes, the timer resets.
//!    (Mirrors: audio_recorder.py L2141-2253)

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, Sender};
use tracing::{debug, info, warn};
use webrtc_vad::{Vad, VadMode};

use super::{
    AudioChunk, AudioSource, MIN_RECORDING_MS, POST_SPEECH_SILENCE_MS,
    PRE_ROLL_SECONDS, SAMPLE_RATE, VAD_CHUNK_SAMPLES, WEBRTC_MODE,
};

/// The state of the VAD state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VadState {
    Listening,
    Recording,
}

/// Accumulated recording frames ready for transcription.
pub struct RecordingResult {
    pub samples: Vec<f32>,
    pub source: AudioSource,
    pub timestamp_ms: u64,
}

/// Runs the VAD + pre-roll buffer pipeline for a single audio source.
pub fn start_vad_pipeline(
    source: AudioSource,
    enabled: Arc<AtomicBool>,
    shutdown: Arc<AtomicBool>,
    chunk_rx: Receiver<AudioChunk>,
    result_tx: Sender<RecordingResult>,
) -> std::thread::JoinHandle<()> {
    let source_name = match source {
        AudioSource::Mic => "mic",
        AudioSource::System => "system",
    };

    std::thread::Builder::new()
        .name(format!("vad-{source_name}"))
        .spawn(move || {
            if let Err(e) = run_vad_loop(source, enabled, shutdown, chunk_rx, result_tx) {
                warn!("VAD pipeline ({source_name}) exited: {e:#}");
            }
        })
        .expect("Failed to spawn VAD thread")
}

fn run_vad_loop(
    source: AudioSource,
    enabled: Arc<AtomicBool>,
    shutdown: Arc<AtomicBool>,
    chunk_rx: Receiver<AudioChunk>,
    result_tx: Sender<RecordingResult>,
) -> anyhow::Result<()> {
    let source_name = match source {
        AudioSource::Mic => "mic",
        AudioSource::System => "system",
    };

    let mut vad = Vad::new();
    let mode = match WEBRTC_MODE {
        0 => VadMode::Quality,
        1 => VadMode::LowBitrate,
        2 => VadMode::Aggressive,
        _ => VadMode::VeryAggressive,
    };
    vad.set_mode(mode);

    let pre_roll_capacity =
        ((SAMPLE_RATE as f32 / VAD_CHUNK_SAMPLES as f32) * PRE_ROLL_SECONDS) as usize;
    let mut pre_roll: VecDeque<Vec<f32>> = VecDeque::with_capacity(pre_roll_capacity);

    let mut state = VadState::Listening;
    let mut recording_frames: Vec<f32> = Vec::new();
    let mut recording_start_ts: u64 = 0;
    let mut recording_start_instant = Instant::now();
    let mut silence_start: Option<Instant> = None;

    let post_speech_silence = Duration::from_millis(POST_SPEECH_SILENCE_MS);
    let min_recording_len = Duration::from_millis(MIN_RECORDING_MS);

    info!("[{source_name}] VAD pipeline started (pre_roll={pre_roll_capacity} chunks)");

    loop {
        if shutdown.load(Ordering::Relaxed) { break; }

        let chunk = match chunk_rx.recv_timeout(Duration::from_millis(200)) {
            Ok(c) => c,
            Err(crossbeam_channel::RecvTimeoutError::Timeout) => continue,
            Err(crossbeam_channel::RecvTimeoutError::Disconnected) => break,
        };

        // Skip if this pipeline is disabled via toggle
        if !enabled.load(Ordering::Relaxed) {
            if !pre_roll.is_empty() || !recording_frames.is_empty() {
                debug!("[{source_name}] Disabled — clearing buffers");
                pre_roll.clear();
                recording_frames.clear();
                state = VadState::Listening;
            }
            continue;
        }

        let is_speech = webrtc_check_speech(&mut vad, &chunk.samples);

        match state {
            VadState::Listening => {
                if pre_roll.len() >= pre_roll_capacity {
                    pre_roll.pop_front();
                }
                pre_roll.push_back(chunk.samples.clone());

                if is_speech {
                    info!("[{source_name}] Speech detected");
                    recording_frames.clear();
                    for buffered_chunk in pre_roll.drain(..) {
                        recording_frames.extend_from_slice(&buffered_chunk);
                    }
                    recording_frames.extend_from_slice(&chunk.samples);
                    recording_start_ts = chunk.timestamp_ms;
                    recording_start_instant = Instant::now();
                    silence_start = None;
                    state = VadState::Recording;
                }
            }

            VadState::Recording => {
                recording_frames.extend_from_slice(&chunk.samples);
                if is_speech {
                    silence_start = None;
                } else {
                    if silence_start.is_none() {
                        silence_start = Some(Instant::now());
                    }
                    let silence_duration = silence_start.unwrap().elapsed();
                    let recording_duration = recording_start_instant.elapsed();

                    if silence_duration >= post_speech_silence
                        && recording_duration >= min_recording_len
                    {
                        info!("[{source_name}] Speech ended — finalizing");
                        let result = RecordingResult {
                            samples: std::mem::take(&mut recording_frames),
                            source,
                            timestamp_ms: recording_start_ts,
                        };
                        if result_tx.send(result).is_err() {
                            break;
                        }
                        silence_start = None;
                        state = VadState::Listening;
                    }
                }
            }
        }
    }
    
    debug!("[{source_name}] VAD thread exiting");
    Ok(())
}

fn webrtc_check_speech(vad: &mut Vad, samples: &[f32]) -> bool {
    let frame_size = 160;
    for frame in samples.chunks(frame_size) {
        if frame.len() < frame_size { break; }
        let i16_frame: Vec<i16> = frame.iter()
            .map(|&s| (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)
            .collect();
        if matches!(vad.is_voice_segment(&i16_frame), Ok(true)) {
            return true;
        }
    }
    false
}
