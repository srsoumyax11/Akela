Akela — STT Engine Implementation Plan
EPIC 4 — STT Engine
Goal:
Create a low-latency local speech-to-text pipeline
using Whisper.cpp with real-time streaming support.The STT engine must:
- work offline
- support continuous transcription
- support long meetings
- support low-end laptops
- support streaming transcript updates
- support future Linux compatibility
FINAL STT STACK
Primary STT Engine:
- whisper.cppRust Binding:
- whisper-rsAudio Input:
- CPAL
- WASAPI loopbackBuffering:
- ringbufRuntime:
- tokioResampling:
- rubatoLogging:
- tracing
REQUIRED DEPENDENCIES
Cargo.toml[dependencies]tokio = { version = "1", features = ["full"] }whisper-rs = "0.11"cpal = "0.15"wasapi = "0.23"ringbuf = "0.3"rubato = "0.15"tracing = "0.1"
tracing-subscriber = "0.3"serde = { version = "1", features = ["derive"] }anyhow = "1"
thiserror = "1"
FOLDER STRUCTURE
src-tauri/src/stt/├── mod.rs
├── whisper_engine.rs
├── inference_worker.rs
├── chunk_processor.rs
├── transcript_cleaner.rs
├── transcript_merger.rs
├── transcript_stream.rs
├── stt_events.rs
├── stt_config.rs
├── resampler.rs
└── utils.rs
FEATURE 4.1 — Whisper Integration

Task — Integrate whisper.cpp
Library Used:
- whisper-rsPurpose:
Rust bindings for whisper.cpp.Why:
- lightweight
- local inference
- no Python dependency
- low memory usage
- real-time capable
Step 1 — Download Models
Recommended Models:V1 Fastest:
- ggml-tiny.en.binV1 Better Accuracy:
- ggml-base.en.binStore models in:/models/whisper/
Step 2 — Initialize Whisper Context
File:
whisper_engine.rsMain APIs:WhisperContext::new()
create_state()Purpose:
- load model
- initialize inference engine
- allocate inference memory
Step 3 — Configure English Mode
Recommended Settings:language = "en"
translate = falseReason:
- lower latency
- lower RAM
- better English accuracy
Step 4 — Configure Streaming
Use:
continuous chunk inferenceRecommended Chunk Size:
500ms–1500msReason:
- balances latency and accuracy
- stable CPU usage
Task — Inference Pipeline
Goal:
Convert live audio chunks into transcript text.
Pipeline Architecture
Audio Input
    ↓
Ring Buffer
    ↓
Chunk Builder
    ↓
Resampler
    ↓
Whisper Queue
    ↓
Inference Worker
    ↓
Transcript Events
Step 1 — Audio Chunk Processing
Input:
48kHz audio from Windows.Whisper Requires:
16kHz mono float audio.Use:
rubatoPurpose:
real-time audio resampling.
Step 2 — Build STT Worker
File:
inference_worker.rsPurpose:
dedicated inference thread/task.Important Rule:
Never block audio capture thread.Use:
tokio::spawn()
Step 3 — Generate Transcript
Main APIs:state.full()
full_n_segments()
full_get_segment_text()Purpose:
- run inference
- retrieve transcript segments
Step 4 — Low Latency Optimization
Recommended:Tiny Model:
- fastestCPU Threads:
- dynamic based on CPU countStreaming:
- incremental inferenceAvoid:
- giant audio chunks
- blocking operations
FEATURE 4.2 — Transcript Processing
Goal:
Clean and stabilize transcript output.
Task — Transcript Cleanup
File:
transcript_cleaner.rs
Step 1 — Remove Artifacts
Remove:
- repeated words
- random symbols
- invalid tokens
- duplicated segmentsExample:BAD:
"hello hello hello"GOOD:
"hello"
Step 2 — Normalize Punctuation
Add:
- sentence capitalization
- commas
- periodsReason:
better AI context quality.
Step 3 — Merge Transcript Chunks
Problem:
streaming inference creates split sentences.Solution:
merge nearby chunks.Example:Chunk 1:
"what is the"Chunk 2:
"cost of banana"Final:
"what is the cost of banana"
Task — Live Transcript Ticker
Goal:
stream transcript into overlay in real time.
Step 1 — Continuous Scrolling
Overlay behavior:
- newest text visible
- old text shifts left
- smooth animation
Step 2 — Overlay Rendering
Render:
- speaker transcript
- user transcriptLabels:
[user]
[speaker]
Step 3 — Real-Time Updates
Use:
tokio broadcast channels.Example:TranscriptGenerated
TranscriptUpdated
FEATURE 4.3 — Performance Optimization
Goal:
Reduce:
- CPU usage
- RAM usage
- latency spikes
Task — CPU Optimization

Step 1 — Quantization Tuning
Recommended:
Q5/Q8 models.Reason:
smaller memory footprint.
Step 2 — Thread Optimization
Set threads dynamically.Recommended:2–4 threads on low-end laptops.Avoid:
using all CPU cores.
Step 3 — Chunk Size Tuning
Small chunks:
- lower latency
- higher CPU usageLarge chunks:
- better accuracy
- higher delayRecommended:
800ms–1200ms
Task — Memory Optimization

Step 1 — Buffer Cleanup
Never:
store infinite audio.Use:
rolling buffers only.
Step 2 — Model Memory Tuning
Load:
single shared Whisper context.Avoid:
multiple model instances.
Step 3 — Stream Recycling
Recycle:
- audio buffers
- transcript buffers
- inference queuesAvoid:
constant allocations.
EVENT SYSTEM
Use:
tokio::sync::mpscExample Events:AudioChunkReady
TranscriptGenerated
TranscriptUpdated
InferenceError
THREADING MODEL
Main Runtime
│
├── Audio Capture Task
├── Chunk Builder Task
├── Resampler Task
├── STT Worker Task
├── Transcript Cleaner Task
└── Overlay Update Task
IMPORTANT ENGINEERING RULES
1.
Never block audio capture.2.
Keep buffers bounded.3.
Do not store raw audio forever.4.
Always separate:
- audio thread
- inference thread
- UI thread5.
Recover automatically after failures.
OPTIONAL FUTURE IMPROVEMENTS
Future V2 Features:- Voice Activity Detection (VAD)
- GPU acceleration
- Multi-language support
- Speaker diarization
- Process-aware capture
- Semantic transcript search
RECOMMENDED FUTURE LIBRARIES
VAD:
- webrtc-vadGPU Inference:
- whisper.cpp CUDASearch:
- sqlite FTS5
FINAL STT ARCHITECTURE
CPAL + WASAPI
        ↓
Ring Buffer
        ↓
Rubato Resampler
        ↓
Whisper-RS
        ↓
Transcript Cleaner
        ↓
Overlay + SQLite
FINAL ENGINEERING PRINCIPLES
- low latency first
- continuous streaming
- lightweight inference
- fault tolerant
- async-native
- overlay optimized
