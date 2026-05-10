use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{Sample, SampleFormat, StreamConfig};
use std::sync::{Arc, Mutex};
use tracing::{error, info};
use crate::audio::audio_buffer::AudioBuffer;

pub struct MicCapture {
    stream: Option<cpal::Stream>,
}

impl MicCapture {
    pub fn new() -> Self {
        Self { stream: None }
    }

    pub fn start_capture(
        &mut self,
        device: &cpal::Device,
        buffer: Arc<Mutex<AudioBuffer>>,
    ) -> anyhow::Result<()> {
        let config: StreamConfig = device.default_input_config()?.into();
        let sample_format = device.default_input_config()?.sample_format();

        info!(
            "Starting Mic Capture: {} channels, {} Hz, format: {:?}",
            config.channels, config.sample_rate.0, sample_format
        );

        let stream = match sample_format {
            SampleFormat::F32 => self.build_stream::<f32>(device, &config, buffer)?,
            SampleFormat::I16 => self.build_stream::<i16>(device, &config, buffer)?,
            SampleFormat::U16 => self.build_stream::<u16>(device, &config, buffer)?,
            _ => return Err(anyhow::anyhow!("Unsupported sample format")),
        };

        stream.play()?;
        self.stream = Some(stream);
        Ok(())
    }

    fn build_stream<T>(
        &self,
        device: &cpal::Device,
        config: &StreamConfig,
        buffer: Arc<Mutex<AudioBuffer>>,
    ) -> anyhow::Result<cpal::Stream>
    where
        T: Sample + cpal::SizedSample + Into<f32>,
    {
        let stream = device.build_input_stream(
            config,
            move |data: &[T], _| {
                let mut buffer_lock = buffer.lock().unwrap();
                for &sample in data {
                    let _ = buffer_lock.push(sample.into());
                }
            },
            |err: cpal::StreamError| error!("Mic Stream Error: {}", err),
            None,
        )?;

        Ok(stream)
    }

    #[allow(dead_code)]
    pub fn stop_capture(&mut self) {
        self.stream = None;
    }
}
