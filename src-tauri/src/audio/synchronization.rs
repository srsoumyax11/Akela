use std::time::Instant;
use crate::audio::audio_events::{AudioChunk, AudioSource};

pub struct ChunkBuilder {
    source: AudioSource,
    #[allow(dead_code)]
    sample_rate: u32,
    buffer: Vec<f32>,
    chunk_size_samples: usize,
}

impl ChunkBuilder {
    pub fn new(source: AudioSource, sample_rate: u32, chunk_duration_ms: u32) -> Self {
        let chunk_size_samples = (sample_rate as f32 * (chunk_duration_ms as f32 / 1000.0)) as usize;
        Self {
            source,
            sample_rate,
            buffer: Vec::with_capacity(chunk_size_samples),
            chunk_size_samples,
        }
    }

    #[allow(dead_code)]
    pub fn stop_capture(&mut self) {}

    pub fn add_sample(&mut self, sample: f32) -> Option<AudioChunk> {
        self.buffer.push(sample);
        
        if self.buffer.len() >= self.chunk_size_samples {
            let chunk = AudioChunk {
                source: self.source.clone(),
                timestamp: Instant::now(),
                samples: std::mem::replace(&mut self.buffer, Vec::with_capacity(self.chunk_size_samples)),
            };
            Some(chunk)
        } else {
            None
        }
    }
}
