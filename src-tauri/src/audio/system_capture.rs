//! System Audio Loopback Capture Pipeline

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::{Context, Result};
use crossbeam_channel::Sender;
use rubato::{FftFixedIn, Resampler};
use tracing::{debug, error, info, warn};

use super::{AudioChunk, AudioSource, SAMPLE_RATE, VAD_CHUNK_SAMPLES};

/// Starts the system loopback capture pipeline on a dedicated thread.
pub fn start_system_capture(
    chunk_tx: Sender<AudioChunk>,
    shutdown: Arc<AtomicBool>,
) -> Result<std::thread::JoinHandle<()>> {
    let handle = std::thread::Builder::new()
        .name("system-capture".into())
        .spawn(move || {
            if let Err(e) = run_loopback_capture(chunk_tx, shutdown) {
                error!("System capture failed: {e:#}");
            }
        })
        .context("Failed to spawn system capture thread")?;

    Ok(handle)
}

fn run_loopback_capture(chunk_tx: Sender<AudioChunk>, shutdown: Arc<AtomicBool>) -> Result<()> {
    wasapi::initialize_mta().ok().context("COM init failed")?;

    let enumerator = wasapi::DeviceEnumerator::new().context("Failed to create device enumerator")?;
    let device = enumerator.get_default_device(&wasapi::Direction::Render).context("No default render device")?;

    let device_name = device.get_friendlyname().unwrap_or_else(|_| "Unknown".into());
    info!("System loopback device: {device_name}");

    let mut audio_client = device.get_iaudioclient().context("Failed to get IAudioClient")?;
    let mix_format = audio_client.get_mixformat().context("Failed to get mix format")?;

    let device_sr = mix_format.get_samplespersec() as usize;
    let device_channels = mix_format.get_nchannels() as usize;
    let bytes_per_sample = mix_format.get_bitspersample() as usize / 8;
    let bytes_per_frame = bytes_per_sample * device_channels;

    let stream_mode = wasapi::StreamMode::EventsShared {
        autoconvert: true,
        buffer_duration_hns: 200_000,
    };

    audio_client.initialize_client(&mix_format, &wasapi::Direction::Capture, &stream_mode).context("Failed to initialize loopback client")?;
    let capture_client = audio_client.get_audiocaptureclient().context("Failed to get capture client")?;
    let event_handle = audio_client.set_get_eventhandle().context("Failed to get event handle")?;

    audio_client.start_stream().context("Failed to start loopback stream")?;
    info!("System loopback stream started");

    let target_sr = SAMPLE_RATE as usize;
    let resample_chunk = 1024_usize;
    let mut resampler = if device_sr != target_sr {
        Some(FftFixedIn::<f32>::new(device_sr, target_sr, resample_chunk, 1, 1).expect("Failed to create system resampler"))
    } else {
        None
    };

    let mut mono_buf: Vec<f32> = Vec::with_capacity(resample_chunk * 2);
    let mut resampled_accum: Vec<f32> = Vec::with_capacity(target_sr);
    let buffer_frames = audio_client.get_buffer_size().unwrap_or(4096) as usize;
    let mut raw_buf = vec![0u8; buffer_frames * bytes_per_frame];

    loop {
        if shutdown.load(Ordering::Relaxed) {
            info!("System capture shutting down...");
            break;
        }

        if event_handle.wait_for_event(200).is_err() {
            continue;
        }

        match capture_client.read_from_device(&mut raw_buf) {
            Ok((frames_read, _)) => {
                if frames_read == 0 { continue; }
                let frame_count = frames_read as usize;
                for frame_idx in 0..frame_count {
                    let mut sum = 0.0_f32;
                    for ch_idx in 0..device_channels {
                        let offset = (frame_idx * device_channels + ch_idx) * bytes_per_sample;
                        if offset + bytes_per_sample <= raw_buf.len() {
                            let sample = match bytes_per_sample {
                                4 => f32::from_le_bytes([raw_buf[offset], raw_buf[offset + 1], raw_buf[offset + 2], raw_buf[offset + 3]]),
                                2 => i16::from_le_bytes([raw_buf[offset], raw_buf[offset + 1]]) as f32 / i16::MAX as f32,
                                _ => 0.0,
                            };
                            sum += sample;
                        }
                    }
                    mono_buf.push(sum / device_channels as f32);
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
                    None => { resampled_accum.append(&mut mono_buf); }
                }

                while resampled_accum.len() >= VAD_CHUNK_SAMPLES {
                    let samples: Vec<f32> = resampled_accum.drain(..VAD_CHUNK_SAMPLES).collect();
                    let timestamp_ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64;
                    let chunk = AudioChunk { samples, timestamp_ms, source: AudioSource::System };
                    if chunk_tx.send(chunk).is_err() { break; }
                }
            }
            Err(e) => {
                warn!("System capture read error: {e}");
                std::thread::sleep(Duration::from_millis(10));
            }
        }
    }

    drop(audio_client);
    debug!("System capture thread exited");
    Ok(())
}
