# Akela — Audio Engine Implementation Master Plan

# 1. Objective

Build a:

- low-latency
- fault-tolerant
- real-time
- Windows-native
- dual-source
- streaming audio engine

for:
- microphone capture
- system audio capture
- speech transcription
- contextual AI processing

The engine must support:

- long meeting sessions
- low-end laptops
- low RAM usage
- automatic recovery
- asynchronous processing
- future Linux expansion

---

# 2. Final Audio Architecture

```text
Microphone (User)
    ↓
CPAL Input Stream
    ↓
Ring Buffer
    ↓
Chunk Builder
    ↓
STT Queue
    ↓
Whisper.cpp
    ↓
Transcript Stream


System Audio (Speaker)
    ↓
WASAPI Loopback
    ↓
Ring Buffer
    ↓
Chunk Builder
    ↓
STT Queue
    ↓
Whisper.cpp
    ↓
Transcript Stream
```

---

# 3. Final Technology Stack

| Area | Technology |
|---|---|
| Runtime | Tokio |
| Mic Capture | CPAL |
| System Audio | wasapi-rs |
| Buffering | ringbuf |
| STT | whisper-rs |
| Event System | tokio channels |
| Logging | tracing |
| Serialization | serde |
| Config | confy |
| Timing | chrono |
| Error Handling | anyhow + thiserror |

---

# 4. Required Rust Crates

# Core Runtime

## tokio

### Purpose
Async runtime.

### Why Needed
Handles:
- streaming tasks
- audio tasks
- STT tasks
- event pipelines
- concurrent processing

### Dependency

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

### Important APIs

```rust
#[tokio::main]
tokio::spawn()
tokio::sync::mpsc
```

---

# Audio Input Layer

## cpal

### Purpose
Cross-platform audio abstraction.

### Why Needed
Handles:
- microphone capture
- device enumeration
- default device selection
- audio input stream

### Why We Use It

Advantages:
- stable
- modern
- future Linux support
- low overhead
- Rust-native

### Dependency

```toml
cpal = "0.15"
```

### Important Types

```rust
cpal::Host
cpal::Device
cpal::Stream
cpal::SampleFormat
```

### Important Functions

```rust
host.default_input_device()
device.default_input_config()
device.build_input_stream()
```

### What CPAL Handles For Us

| Feature | Status |
|---|---|
| Default mic detection | Yes |
| Input streaming | Yes |
| Device enumeration | Yes |
| Cross-platform support | Yes |

### What CPAL Does NOT Handle Well

| Missing Feature |
|---|
| Advanced WASAPI loopback |
| Process-specific capture |
| Advanced Windows callbacks |

---

# System Audio Capture

## wasapi-rs

### Purpose
Native Windows WASAPI integration.

### Why Needed
Required for:
- system audio capture
- loopback capture
- Windows-native behavior
- advanced audio APIs

### Dependency

```toml
wasapi = "0.23"
```

### Important Features

| Feature | Support |
|---|---|
| Loopback capture | Yes |
| Device notifications | Yes |
| Event-driven capture | Yes |
| Process loopback | Yes |
| Reconnect handling | Yes |

### Important APIs

```rust
AudioClient
Direction::Render
initialize_client()
```

### Critical WASAPI Flag

```rust
AUDCLNT_STREAMFLAGS_LOOPBACK
```

This allows:

```text
Capture speaker output audio
```

### Future Feature Support

Possible future support:

```rust
new_application_loopback_client(process_id, include_tree)
```

Meaning:
- Zoom-only capture
- Meet-only capture
- Teams-only capture

VERY IMPORTANT future-proofing.

---

# Audio Buffering

## ringbuf

### Purpose
Real-time audio buffering.

### Why Needed
Audio streaming requires:
- lock-free queues
- low latency
- predictable memory usage

### Dependency

```toml
ringbuf = "0.3"
```

### Why Ring Buffers

Better than:
- Vec queues
- Mutex-heavy structures
- standard channels for raw audio

### Benefits

| Benefit | Reason |
|---|---|
| Lock-free | lower latency |
| Fixed memory | predictable RAM |
| Real-time friendly | stable streaming |

### Important Types

```rust
HeapRb
Producer
Consumer
```

### Example Use

```rust
let rb = HeapRb::<f32>::new(48000);
let (producer, consumer) = rb.split();
```

---

# Speech-to-Text

## whisper-rs

### Purpose
Rust bindings for whisper.cpp.

### Why Needed
Provides:
- local STT
- fast inference
- CPU-based inference
- streaming support

### Dependency

```toml
whisper-rs = "0.11"
```

### Why whisper-rs

Advantages:
- Rust-native
- low overhead
- direct bindings
- no Python dependency

### Recommended Model

V1:

```text
Whisper Tiny/Base
```

### Recommended Quantization

```text
INT8 quantized
```

### Important Types

```rust
WhisperContext
WhisperState
FullParams
```

### Important APIs

```rust
WhisperContext::new()
state.full()
state.full_n_segments()
```

### Important Configuration

| Setting | Value |
|---|---|
| Language | English |
| Translation | Disabled |
| Threads | CPU optimized |
| Real-time mode | Enabled |

---

# Logging System

## tracing

### Purpose
Structured async logging.

### Dependencies

```toml
tracing = "0.1"
tracing-subscriber = "0.3"
```

### Why Needed
Critical for debugging:
- audio drops
- WASAPI failures
- STT latency
- queue overflow

### Important Macros

```rust
info!()
warn!()
error!()
debug!()
```

---

# Error Handling

## anyhow

### Purpose
Generic application errors.

### Dependency

```toml
anyhow = "1"
```

---

## thiserror

### Purpose
Custom typed errors.

### Dependency

```toml
thiserror = "1"
```

### Example

```rust
#[derive(Error, Debug)]
pub enum AudioError {
    #[error("device disconnected")]
    DeviceDisconnected,
}
```

---

# Serialization

## serde

### Purpose
Configuration + event serialization.

### Dependency

```toml
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

---

# Configuration System

## confy

### Purpose
Store local app settings.

### Dependency

```toml
confy = "0.6"
```

### Used For

| Setting |
|---|
| Selected microphone |
| Overlay opacity |
| Startup settings |
| AI provider |

---

# 5. Folder Structure

```text
src-tauri/src/audio/
│
├── mod.rs
├── mic_capture.rs
├── system_capture.rs
├── device_manager.rs
├── audio_buffer.rs
├── synchronization.rs
├── source_classifier.rs
├── recovery.rs
├── audio_events.rs
├── config.rs
└── utils.rs
```

---

# 6. Module Implementation Plan

# Phase 1 — Device Enumeration

## Goal
Detect:
- default microphone
- default speaker

---

## Implement

### CPAL Device Enumeration

```rust
let host = cpal::default_host();
```

### Get Default Mic

```rust
host.default_input_device()
```

### Enumerate Devices

```rust
host.input_devices()
```

---

# Phase 2 — Microphone Stream

## Goal
Create continuous mic stream.

---

## Implement

### Create Input Stream

```rust
device.build_input_stream()
```

### Configure:

| Setting | Value |
|---|---|
| Channels | mono/stereo |
| Sample Rate | 16000/48000 |
| Buffer Size | low latency |

---

## Output

Mic chunks pushed into:

```text
Ring Buffer
```

---

# Phase 3 — WASAPI Loopback

# MOST IMPORTANT PHASE

---

## Goal
Capture speaker output audio.

---

## Implement

### Initialize WASAPI Client

### Enable Loopback Mode

### Start Event-Driven Capture

### Push Audio Into Ring Buffer

---

## Critical Concepts

| Concept | Description |
|---|---|
| Render Device | speaker output |
| Loopback | capture output audio |
| Shared Mode | normal Windows mode |
| Event Callback | async notifications |

---

# Phase 4 — Audio Buffers

## Goal
Stabilize streaming.

---

## Buffer Architecture

```text
Audio Input
    ↓
Ring Buffer
    ↓
Chunk Builder
    ↓
STT Queue
```

---

## Chunk Size

Recommended:

```text
500ms–1500ms
```

Reason:
- stable Whisper inference
- low latency
- good CPU usage

---

# Phase 5 — Synchronization

## Goal
Keep streams aligned.

---

## Problem
Mic and loopback streams:
- arrive independently
- drift over time
- have different timing

---

## Solution
Attach:

```text
timestamps
```

to every chunk.

---

## Required Metadata

```rust
struct AudioChunk {
    source: AudioSource,
    timestamp: Instant,
    samples: Vec<f32>,
}
```

---

# Phase 6 — STT Pipeline

## Goal
Convert audio → text.

---

## Pipeline

```text
Audio Chunk
    ↓
Whisper Queue
    ↓
Inference Worker
    ↓
Transcript Event
```

---

## Recommended Architecture

Separate:

```text
Capture Thread
Inference Thread
Overlay Thread
```

Never block audio capture.

---

# Phase 7 — Event System

## Goal
Decouple modules.

---

## Recommended

```rust
tokio::sync::mpsc
```

---

## Example Events

```rust
enum AudioEvent {
    ChunkReady(AudioChunk),
    DeviceDisconnected,
    DeviceRecovered,
}
```

---

# Phase 8 — Recovery System

## Goal
Prevent crashes.

---

## Must Handle

| Scenario |
|---|
| Mic unplugged |
| Bluetooth disconnect |
| Speaker switch |
| WASAPI reset |
| Sleep/wake |

---

## Recovery Strategy

```text
Failure
    ↓
Detect
    ↓
Destroy Stream
    ↓
Reinitialize
    ↓
Resume
```

---

# 7. Performance Targets

| Area | Target |
|---|---|
| Mic Latency | <100ms |
| Loopback Latency | <150ms |
| STT Delay | <1 sec |
| RAM Usage | low |
| CPU Usage | low |

---

# 8. Recommended Sample Rates

| Usage | Sample Rate |
|---|---|
| Capture | 48000 |
| Whisper Processing | 16000 |

---

# Why

Most Windows devices use:

```text
48000Hz
```

Whisper prefers:

```text
16000Hz
```

So resampling is required.

---

# Recommended Resampler

## rubato

### Dependency

```toml
rubato = "0.15"
```

### Purpose
Real-time audio resampling.

---

# 9. Threading Model

## Recommended

```text
Main Runtime
│
├── Mic Capture Task
├── Loopback Capture Task
├── Chunk Builder Task
├── STT Worker Task
├── Overlay Event Task
└── Persistence Task
```

---

# 10. Recommended Audio Flow

```text
Mic/System Audio
    ↓
Capture Stream
    ↓
Ring Buffer
    ↓
Chunk Builder
    ↓
Resampler
    ↓
Whisper Queue
    ↓
Whisper Inference
    ↓
Transcript Event
    ↓
Overlay + DB
```

---

# 11. Optional Advanced Knowledge

# A. Event-Driven WASAPI

Preferred over polling.

Benefits:
- lower CPU usage
- lower latency
- cleaner synchronization

---

# B. Shared vs Exclusive Mode

## Shared Mode
Recommended.

Allows:
- coexist with Zoom/Meet
- coexist with Discord
- coexist with browsers

---

## Exclusive Mode
Avoid.

Problems:
- breaks user audio
- conflicts with meeting apps

---

# C. Echo Cancellation

Future possibility.

Useful if:
- mic hears speaker audio
- transcript duplication occurs

---

# D. Voice Activity Detection (VAD)

Future optimization.

Benefits:
- skip silence
- lower CPU usage
- lower API usage

Recommended future crate:

```text
webrtc-vad
```

---

# E. Process-Aware Capture

Future V2 feature.

Capture ONLY:
- Zoom
- Meet
- Teams

using:

```rust
new_application_loopback_client()
```

---

# 12. Engineering Risks

| Risk | Severity |
|---|---|
| WASAPI resets | Critical |
| Bluetooth instability | High |
| Audio drift | Medium |
| Queue overflow | Medium |
| Whisper CPU spikes | Medium |

---

# 13. QA Checklist

## Device Tests

- unplug headset
- reconnect headset
- switch microphone
- switch speaker
- Bluetooth reconnect

---

## Stress Tests

- Zoom + Discord
- long meetings
- low-end laptops
- browser meetings

---

## Performance Tests

- RAM monitoring
- CPU monitoring
- latency tracking
- queue overflow tests

---

# 14. Final Engineering Principles

```text
Never block audio capture.
Always recover automatically.
Keep buffers bounded.
Optimize for latency first.
```

---

# 15. Final Recommended Dependencies

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
cpal = "0.15"
wasapi = "0.23"
ringbuf = "0.3"
whisper-rs = "0.11"
rubato = "0.15"
tracing = "0.1"
tracing-subscriber = "0.3"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
confy = "0.6"
chrono = "0.4"
anyhow = "1"
thiserror = "1"
```

---

# 16. Final Audio Stack Decision

```text
CPAL
    +
WASAPI-RS
    +
Ring Buffers
    +
Whisper-RS
    +
Tokio
```

This stack is approved for:
- prototype implementation
- production MVP
- low-latency streaming
- future Linux expansion
- long-session stability.
