# Akela — Software Architecture Document (SAD)

## Project Overview

### Project Name

```text
Akela
```

Inspired by:

* Akela from *The Jungle Book*
* intelligent
* observant
* always present
* guiding companion

---

# 1. System Overview

## Product Definition

Akela is a:

```text
Local-first Windows overlay AI meeting assistant
```

designed for:

* researchers
* students
* professionals
* productivity users

The application:

* continuously transcribes meeting audio locally
* stores transcripts locally
* provides contextual AI assistance through a floating overlay capsule
* streams AI responses in real time
* operates without dedicated backend infrastructure

---

# 2. Core Product Principles

| Principle       | Description                             |
| --------------- | --------------------------------------- |
| Local-first     | transcripts and processing remain local |
| Open-source     | MIT licensed                            |
| Overlay-native  | no traditional desktop workflow         |
| Low-latency     | optimized for real-time use             |
| Lightweight     | optimized for standard laptops          |
| Non-intrusive   | passive overlay behavior                |
| User-controlled | manual AI trigger                       |
| No backend      | user-owned API keys                     |

---

# 3. High-Level Architecture

```text
+--------------------------------------------------+
|                    Akela                         |
+--------------------------------------------------+
|                                                  |
|  Tauri Shell                                     |
|  ├── Settings UI                                 |
|  ├── History UI                                  |
|  ├── Export UI                                   |
|  └── Tray Integration                            |
|                                                  |
|  Native Overlay Engine                           |
|  ├── Capsule Renderer                            |
|  ├── Transcript Ticker                           |
|  ├── AI Response Renderer                        |
|  ├── Overlay Expansion                           |
|  └── Transparency/Blur                           |
|                                                  |
|  Audio Engine                                    |
|  ├── Microphone Capture                          |
|  ├── WASAPI Loopback Capture                     |
|  └── Audio Device Management                     |
|                                                  |
|  STT Engine                                      |
|  ├── Whisper.cpp Integration                     |
|  ├── Streaming Transcription                     |
|  └── Transcript Chunking                         |
|                                                  |
|  AI Engine                                       |
|  ├── OpenAI Provider                             |
|  ├── Gemini Provider                             |
|  ├── NVIDIA Provider                             |
|  └── Streaming Response Pipeline                 |
|                                                  |
|  Persistence Engine                              |
|  ├── SQLite                                      |
|  ├── Transcript Storage                          |
|  ├── Session Storage                             |
|  └── Export System                               |
|                                                  |
+--------------------------------------------------+
```

---

# 4. Architecture Style

## Selected Pattern

```text
Modular Monolith
```

Characteristics:

* single executable
* single deployment
* internally separated modules
* shared memory space
* async event-driven communication

---

# 5. Technology Stack

## Core Stack

| Layer           | Technology         |
| --------------- | ------------------ |
| Language        | Rust               |
| Shell Framework | Tauri              |
| Frontend        | React + TypeScript |
| Runtime         | Tokio              |
| Database        | SQLite             |
| Serialization   | Serde              |

---

## Windows Native Stack

| Area              | Technology         |
| ----------------- | ------------------ |
| Windowing         | Win32 APIs         |
| Overlay Rendering | DirectComposition  |
| Transparency      | Layered Windows    |
| Blur Effects      | DWM / Acrylic      |
| GPU Rendering     | Direct2D / DirectX |

---

## Audio Stack

| Area           | Technology     |
| -------------- | -------------- |
| Audio Capture  | WASAPI         |
| Mic Capture    | Default Device |
| System Capture | Loopback Audio |

---

## AI Stack

| Area      | Technology             |
| --------- | ---------------------- |
| STT       | Whisper.cpp            |
| AI APIs   | OpenAI-compatible APIs |
| Streaming | SSE/WebSockets         |

---

# 6. Overlay Architecture

## Overlay Type

```text
Floating Capsule Overlay
```

Characteristics:

* frameless
* transparent
* always-on-top
* hidden from taskbar
* hidden from Alt+Tab
* movable
* expandable
* keyboard navigable
* non-focus stealing

---

# 7. Overlay States

## State Machine

```text
Hidden
   ↓
Compact Capsule
   ↓
Expanded Overlay
   ↓
Collapsed
```

---

## Compact State

Contains:

* transcript ticker
* Help Me button
* mic indicator
* speaker indicator
* expand control
* settings icon

---

## Expanded State

Contains:

* transcript history
* AI responses
* session controls
* exports
* settings

---

# 8. Audio Pipeline

## Audio Sources

### Microphone

```text
Windows default microphone device
```

### System Audio

```text
WASAPI loopback capture
```

---

## Source Classification

| Source       | Classification |
| ------------ | -------------- |
| Microphone   | User           |
| System Audio | Speaker        |

No advanced diarization in V1.

---

# 9. STT Architecture

## Selected Model

```text
Whisper Tiny/Base
```

Priority:

```text
Maximum Speed
```

Goals:

* low CPU usage
* low latency
* broad laptop compatibility

---

## STT Pipeline

```text
Audio Stream
    ↓
Chunk Buffer
    ↓
Whisper Inference
    ↓
Transcript Cleanup
    ↓
Transcript Storage
    ↓
Overlay Rendering
```

---

# 10. AI Interaction Model

## User Triggered

### Single Click

```text
Help Me
```

Behavior:

* send recent transcript context
* ~40–50 words/chunks

---

### Press and Hold

Behavior:

* accumulate larger transcript window
* send expanded context

---

# 11. AI Provider Architecture

## Provider Abstraction

```rust
trait AIProvider {
    async fn stream_response(...);
}
```

Supported:

* OpenAI
* Gemini
* NVIDIA APIs

---

# 12. Persistence Architecture

## Database

```text
SQLite
```

Local-only storage.

---

## Tables

### sessions

| Field         | Type      |
| ------------- | --------- |
| id            | UUID      |
| title         | TEXT      |
| description   | TEXT      |
| system_prompt | TEXT      |
| created_at    | TIMESTAMP |

---

### transcripts

| Field      | Type      |
| ---------- | --------- |
| id         | UUID      |
| session_id | UUID      |
| source     | TEXT      |
| text       | TEXT      |
| timestamp  | TIMESTAMP |

---

### ai_requests

| Field      | Type      |
| ---------- | --------- |
| id         | UUID      |
| session_id | UUID      |
| prompt     | TEXT      |
| response   | TEXT      |
| timestamp  | TIMESTAMP |

---

# 13. Export System

## Supported Formats

### V1

* TXT
* Markdown

### Future

* PDF
* JSON

---

# 14. Settings System

## Configurable Options

| Setting           | Supported |
| ----------------- | --------- |
| API Provider      | Yes       |
| API Key           | Yes       |
| Overlay Opacity   | Yes       |
| Startup Behavior  | Yes       |
| System Prompt     | Yes       |
| Transcript Export | Yes       |

---

# 15. Hotkey System

## Primary Hotkey

```text
Ctrl + Shift + A
```

Functions:

* toggle overlay
* focus overlay controls
* quick interaction

---

# 16. Tray Integration

## Behavior

Closing overlay:

```text
minimize to system tray
```

Application remains active.

---

# 17. Startup Behavior

## Automatic Transcription

On launch:

* initialize audio
* initialize STT
* begin transcript storage
* activate overlay

No manual “start session”.

---

# 18. Performance Targets

| Area           | Target        |
| -------------- | ------------- |
| RAM Usage      | ~1.3GB max    |
| STT Delay      | <1 second     |
| AI First Token | <2 seconds    |
| Overlay FPS    | 60 FPS target |

---

# 19. Security Model

## V1 Security Decisions

| Area             | Decision        |
| ---------------- | --------------- |
| Local Encryption | No              |
| Authentication   | None            |
| Backend Server   | None            |
| User Accounts    | None            |
| API Ownership    | User-owned keys |

---

# 20. Repository Structure

```text
akela/
│
├── frontend/
│   ├── src/
│   └── public/
│
├── src-tauri/
│   ├── audio/
│   ├── stt/
│   ├── ai/
│   ├── overlay/
│   ├── persistence/
│   ├── system/
│   └── main.rs
│
├── native-overlay/
│
├── docs/
│
└── models/
```

---

# 21. Core Engineering Risks

| Area                  | Difficulty |
| --------------------- | ---------- |
| Overlay Rendering     | High       |
| WASAPI Dual Capture   | High       |
| Non-focus Interaction | High       |
| Streaming STT         | Medium     |
| AI Streaming          | Medium     |

---

# 22. Future Roadmap (Post-V1)

## Planned Future Features

### V2

* screen analysis
* OCR integration
* smarter context extraction
* transcript search

### V3

* Linux support
* process-aware audio capture
* advanced AI memory

---

# 23. Final Architecture Decision

## Selected Architecture

```text
Tauri Shell
+
Native Rust Overlay Renderer
+
Local STT
+
Cloud AI APIs
+
SQLite Persistence
```

This architecture is approved for:

* MVP development
* prototype implementation
* repository initialization
* engineering milestone planning.
