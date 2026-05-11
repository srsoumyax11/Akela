# Audio Pipeline Deep Dive

Akela uses a sophisticated multi-threaded audio pipeline to achieve low-latency, high-accuracy transcription of both your microphone and system audio.

---

## The Dual-Stream Architecture

Akela runs two identical, parallel pipelines: one for the **Microphone** and one for **System Loopback**. This allows us to keep the user's voice and the meeting audio separate, which is critical for future AI analysis (knowing who said what).

### Pipeline Flow
1.  **Capture**: `cpal` or `wasapi` grabs raw PCM audio.
2.  **Resampling**: All audio is converted to **16kHz Mono f32**, the format required by Whisper.
3.  **VAD Scout**: A fast WebRTC VAD filter checks for speech every 32ms.
4.  **Buffering**: If speech is detected, we prepend a **1.0s pre-roll buffer** to ensure no words are cut off.
5.  **Inference**: The combined audio chunk is sent to the local Whisper model.
6.  **Emission**: The transcript is emitted to the frontend via Tauri events.

---

## 1. Capture Stage
We use `cpal` for the microphone and a custom `wasapi` loopback implementation for system audio.
- **Buffer size**: 20ms-40ms chunks to minimize latency.
- **Lock-free processing**: We use `ringbuf` to pass audio between the capture callback and the processing thread without blocking.

## 2. Voice Activity Detection (VAD)
We use a two-stage detection strategy inspired by the `RealtimeSTT` project:
- **WebRTC VAD (Scout)**: Runs at Mode 3 (Very Aggressive) to filter out background noise and silence quickly.
- **Post-Speech Silence**: The engine waits for a configurable period (default 400ms-600ms) of absolute silence before finalizing a transcript segment.

## 3. Whisper Inference
The `stt_worker` thread manages the `WhisperContext`.
- **Model**: Defaulting to `base.en` for the best balance of speed and accuracy.
- **Warm-up**: The model is warmed up with silence on startup to eliminate "first-inference lag."
- **Peak Normalization**: Audio is normalized to 0.95 peak volume before inference to ensure consistent results.

---

## Threading Model
Each pipeline (Mic/System) consumes 4 main threads:
1.  **Capture Thread**: Direct interface with hardware.
2.  **VAD/Buffer Thread**: Logic for segmenting audio.
3.  **STT Worker**: Heavy CPU work for Whisper inference.
4.  **Emitter Thread**: Handles the async bridge to the UI.

Total: **8 dedicated audio threads** ensuring zero lag on the main UI thread.
