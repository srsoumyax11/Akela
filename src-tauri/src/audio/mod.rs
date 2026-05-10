pub mod audio_buffer;
pub mod audio_events;
pub mod device_manager;
pub mod mic_capture;
pub mod system_capture;
pub mod synchronization;

use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tracing::{info, error};
use crate::audio::audio_buffer::AudioBuffer;
use crate::audio::device_manager::DeviceManager;
use crate::audio::mic_capture::MicCapture;
use crate::audio::system_capture::SystemCapture;
use crate::audio::synchronization::ChunkBuilder;
use crate::audio::audio_events::{AudioSource, AudioEvent};

pub struct AudioEngine {
    device_manager: DeviceManager,
    mic_capture: MicCapture,
    system_capture: SystemCapture,
    chunk_sender: mpsc::Sender<AudioEvent>,
}

impl AudioEngine {
    pub fn new(chunk_sender: mpsc::Sender<AudioEvent>) -> Self {
        Self {
            device_manager: DeviceManager::new(),
            mic_capture: MicCapture::new(),
            system_capture: SystemCapture::new(),
            chunk_sender,
        }
    }

    #[allow(dead_code)]
    pub fn stop_capture(&mut self) {
        // Implementation would go here
    }

    #[allow(dead_code)]
    pub fn list_input_devices(&self) {
        // Implementation would go here
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        info!("Starting Audio Engine...");

        let sample_rate = 48000;
        let mic_buffer = Arc::new(Mutex::new(AudioBuffer::new(sample_rate * 2))); // 2 seconds buffer
        let loopback_buffer = Arc::new(Mutex::new(AudioBuffer::new(sample_rate * 2)));

        // Start Mic Capture
        if let Some(mic_device) = self.device_manager.get_default_input_device() {
            self.mic_capture.start_capture(&mic_device, mic_buffer.clone())?;
        } else {
            error!("No default microphone found");
        }

        // Start System Loopback
        if let Ok(loopback_device) = self.device_manager.get_default_loopback_device() {
            self.system_capture.start_capture(loopback_device, loopback_buffer.clone())?;
        } else {
            error!("Failed to initialize system loopback");
        }

        let mut mic_chunk_builder = ChunkBuilder::new(AudioSource::Microphone, 48000, 1000); // 1s chunks
        let mut loopback_chunk_builder = ChunkBuilder::new(AudioSource::SystemLoopback, 48000, 1000);

        let sender = self.chunk_sender.clone();

        // Main processing loop
        loop {
            // Process Mic Samples
            {
                let mut mic_lock = mic_buffer.lock().unwrap();
                while let Some(sample) = mic_lock.pop() {
                    if let Some(chunk) = mic_chunk_builder.add_sample(sample) {
                        // info!("Mic Chunk Ready: {:?} (len={})", chunk.timestamp, chunk.samples.len());
                        let _ = sender.try_send(AudioEvent::ChunkReady(chunk));
                    }
                }
            }

            // Process Loopback Samples
            {
                let mut loopback_lock = loopback_buffer.lock().unwrap();
                while let Some(sample) = loopback_lock.pop() {
                    if let Some(chunk) = loopback_chunk_builder.add_sample(sample) {
                        info!("Loopback Chunk Ready: {:?} (len={})", chunk.timestamp, chunk.samples.len());
                        let _ = sender.try_send(AudioEvent::ChunkReady(chunk));
                    }
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }
}

pub async fn init_audio_engine() -> anyhow::Result<mpsc::Receiver<AudioEvent>> {
    let (tx, rx) = mpsc::channel(100);
    
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
            
        rt.block_on(async {
            let mut engine = AudioEngine::new(tx);
            if let Err(e) = engine.run().await {
                error!("Audio Engine Error: {}", e);
            }
        });
    });

    Ok(rx)
}
