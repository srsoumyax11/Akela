use serde::{Serialize, Deserialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioSource {
    Microphone,
    SystemLoopback,
}

#[derive(Debug, Clone)]
pub struct AudioChunk {
    pub source: AudioSource,
    pub timestamp: Instant,
    pub samples: Vec<f32>,
}

#[derive(Debug, Clone)]
pub enum AudioEvent {
    ChunkReady(AudioChunk),
    StreamStarted(AudioSource),
    StreamStopped(AudioSource),
    DeviceChanged(AudioSource, String),
    Error(String),
}
