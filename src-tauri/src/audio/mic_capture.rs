//! Microphone Capture Pipeline

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, StreamConfig};
use crossbeam_channel::Sender;
use ringbuf::{
    traits::{Consumer, Producer, Split},
    HeapRb,
};
use rubato::{FftFixedIn, Resampler};
use tracing::{debug, error, info, warn};

use super::{AudioChunk, AudioSource, SAMPLE_RATE, VAD_CHUNK_SAMPLES};

/// Starts the microphone capture pipeline on a dedicated thread.
pub fn start_mic_capture(
    chunk_tx: Sender<AudioChunk>,
    shutdown: Arc<AtomicBool>,
) -> Result<std::thread::JoinHandle<()>> {
    let handle = std::thread::Builder::new()
        .name("mic-capture".into())
        .spawn(move || {
            if let Err(e) = run_mic_capture(chunk_tx, shutdown) {
                error!("Mic capture failed: {e:#}");
            }
        })
        .context("Failed to spawn mic capture thread")?;

    Ok(handle)
}

fn run_mic_capture(chunk_tx: Sender<AudioChunk>, shutdown: Arc<AtomicBool>) -> Result<()> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .context("No microphone found")?;

    let device_name = device.name().unwrap_or_else(|_| "Unknown".into());
    info!("Mic device: {device_name}");

    let config_range = device
        .supported_input_configs()
        .context("Failed to query mic configs")?
        .filter(|c| c.channels() == 1 || c.channels() == 2)
        .max_by_key(|c| c.max_sample_rate().0)
        .context("No suitable mic config")?;

    let device_rate = config_range.max_sample_rate();
    let channels = config_range.channels();
    let sample_format = config_range.sample_format();

    let stream_config = StreamConfig {
        channels,
        sample_rate: device_rate,
        buffer_size: cpal::BufferSize::Default,
    };

    let ring_capacity = (device_rate.0 as usize) * (channels as usize) * 2;
    let ring = HeapRb::<f32>::new(ring_capacity);
    let (mut producer, mut consumer) = ring.split();

    let err_fn = |err| error!("Mic stream error: {err}");

    let stream = match sample_format {
        SampleFormat::F32 => device.build_input_stream(
            &stream_config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let _ = producer.push_slice(data);
            },
            err_fn,
            None,
        )?,
        SampleFormat::I16 => device.build_input_stream(
            &stream_config,
            move |data: &[i16], _: &cpal::InputCallbackInfo| {
                for &sample in data {
                    let _ = producer.try_push(sample as f32 / i16::MAX as f32);
                }
            },
            err_fn,
            None,
        )?,
        _ => anyhow::bail!("Unsupported sample format: {:?}", sample_format),
    };

    stream.play().context("Failed to start mic stream")?;
    info!("Mic stream started");

    let device_sr = device_rate.0 as usize;
    let target_sr = SAMPLE_RATE as usize;
    let ch = channels as usize;

    let resample_chunk = 1024_usize;
    let mut resampler = if device_sr != target_sr {
        Some(
            FftFixedIn::<f32>::new(device_sr, target_sr, resample_chunk, 1, 1)
                .expect("Failed to create resampler"),
        )
    } else {
        None
    };

    let mut mono_buf: Vec<f32> = Vec::with_capacity(resample_chunk * 2);
    let mut resampled_accum: Vec<f32> = Vec::with_capacity(target_sr);
    let mut read_buf = vec![0.0f32; 4096];

    loop {
        if shutdown.load(Ordering::Relaxed) {
            info!("Mic capture shutting down...");
            break;
        }

        let count = consumer.pop_slice(&mut read_buf);
        if count == 0 {
            std::thread::sleep(Duration::from_millis(10));
            continue;
        }

        if ch == 2 {
            for pair in read_buf[..count].chunks(2) {
                let mono = if pair.len() == 2 { (pair[0] + pair[1]) * 0.5 } else { pair[0] };
                mono_buf.push(mono);
            }
        } else {
            mono_buf.extend_from_slice(&read_buf[..count]);
        }

        match &mut resampler {
            Some(rs) => {
                while mono_buf.len() >= resample_chunk {
                    let input_chunk: Vec<f32> = mono_buf.drain(..resample_chunk).collect();
                    if let Ok(output) = rs.process(&vec![input_chunk], None) {
                        if let Some(ch0) = output.first() {
                            resampled_accum.extend_from_slice(ch0);
                        }
                    }
                }
            }
            None => {
                resampled_accum.append(&mut mono_buf);
            }
        }

        while resampled_accum.len() >= VAD_CHUNK_SAMPLES {
            let samples: Vec<f32> = resampled_accum.drain(..VAD_CHUNK_SAMPLES).collect();
            let timestamp_ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64;
            let chunk = AudioChunk { samples, timestamp_ms, source: AudioSource::Mic };

            if chunk_tx.send(chunk).is_err() {
                break;
            }
        }
    }

    drop(stream);
    debug!("Mic capture thread exited");
    Ok(())
}
