use wasapi::{DeviceEnumerator, Direction, StreamMode};
use std::sync::{Arc, Mutex};
use tracing::{error, info};
use crate::audio::audio_buffer::AudioBuffer;

pub struct SystemCapture {
    is_running: Arc<Mutex<bool>>,
}

impl SystemCapture {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start_capture(
        &mut self,
        _device: wasapi::Device,
        buffer: Arc<Mutex<AudioBuffer>>,
    ) -> anyhow::Result<()> {
        let mut is_running = self.is_running.lock().unwrap();
        if *is_running {
            return Ok(());
        }
        *is_running = true;
        
        let is_running_clone = self.is_running.clone();

        std::thread::spawn(move || {
            // Initialize COM for this thread
            let _ = wasapi::initialize_mta();

            // We capture from the default render device in loopback mode
            let enumerator = match DeviceEnumerator::new() {
                Ok(e) => e,
                Err(e) => {
                    error!("Failed to create DeviceEnumerator: {}", e);
                    return;
                }
            };
            let device = match enumerator.get_default_device(&Direction::Render) {
                Ok(d) => d,
                Err(e) => {
                    error!("Failed to get default device: {}", e);
                    return;
                }
            };

            let mut client = match device.get_iaudioclient() {
                Ok(c) => c,
                Err(e) => {
                    error!("Failed to get AudioClient: {}", e);
                    return;
                }
            };

            let format = match client.get_mixformat() {
                Ok(f) => f,
                Err(e) => {
                    error!("Failed to get mix format: {}", e);
                    return;
                }
            };

            info!("Starting System Loopback Capture: {:?}", format);

            // In wasapi 0.23, initialize_client takes 3 arguments: format, direction, stream_mode
            // For loopback, we use EventsShared mode
            let mode = StreamMode::EventsShared {
                autoconvert: true,
                buffer_duration_hns: 1000000, // 100ms buffer
            };
            if let Err(e) = client.initialize_client(
                &format,
                &Direction::Capture,
                &mode,
            ) {
                error!("Failed to initialize client: {}", e);
                return;
            }

            let capture_client = match client.get_audiocaptureclient() {
                Ok(c) => c,
                Err(e) => {
                    error!("Failed to get CaptureClient: {}", e);
                    return;
                }
            };

            let h_event = match client.set_get_eventhandle() {
                Ok(h) => h,
                Err(e) => {
                    error!("Failed to set event handle: {}", e);
                    return;
                }
            };

            if let Err(e) = client.start_stream() {
                error!("Failed to start stream: {}", e);
                return;
            }

            while *is_running_clone.lock().unwrap() {
                if h_event.wait_for_event(1000).is_err() {
                    continue;
                }

                // In 0.23, we use get_next_packet_size
                while let Ok(Some(packet_size)) = capture_client.get_next_packet_size() {
                    if packet_size == 0 { break; }
                    
                    let mut data = vec![0u8; (packet_size * format.get_blockalign() as u32) as usize];
                    if let Err(e) = capture_client.read_from_device(&mut data) {
                        error!("Read error: {}", e);
                        break;
                    }

                    // Convert raw bytes to f32
                    let samples: &[f32] = unsafe {
                        std::slice::from_raw_parts(
                            data.as_ptr() as *const f32,
                            data.len() / 4
                        )
                    };

                    let mut buffer_lock = buffer.lock().unwrap();
                    for &sample in samples {
                        let _ = buffer_lock.push(sample);
                    }
                }
            }
            let _ = client.stop_stream();
        });

        Ok(())
    }

    #[allow(dead_code)]
    pub fn stop_capture(&mut self) {
        let mut is_running = self.is_running.lock().unwrap();
        *is_running = false;
    }
}
