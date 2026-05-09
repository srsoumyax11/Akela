# Akela — Module Breakdown Document

## Overview

Akela follows a:

```text id="module-arch"
Modular Monolith Architecture
```

Meaning:

* single executable
* internally isolated modules
* event-driven communication
* shared runtime
* shared memory space

Each module is:

* independently testable
* logically isolated
* asynchronously connected

---

# Core Runtime Topology

```text id="runtime-topology"
+--------------------------------------------------+
|                  Akela Runtime                   |
+--------------------------------------------------+
|                                                  |
|  Overlay Module                                  |
|  Audio Module                                    |
|  STT Module                                      |
|  AI Module                                       |
|  Persistence Module                              |
|  Session Module                                  |
|  Export Module                                   |
|  System Integration Module                       |
|  Config Module                                   |
|  Event Bus                                       |
|                                                  |
+--------------------------------------------------+
```

---

# 1. Overlay Module

## Purpose

Responsible for:

* floating capsule UI
* overlay rendering
* transparency
* animations
* AI response rendering
* transcript ticker

---

## Responsibilities

| Responsibility      | Description            |
| ------------------- | ---------------------- |
| Overlay Rendering   | capsule drawing        |
| Transparency        | acrylic/glass effects  |
| Expansion States    | compact/expanded       |
| Transcript Display  | live transcript stream |
| AI Rendering        | streaming word output  |
| User Interaction    | buttons/hotkeys        |
| Position Management | movable overlay        |
| Opacity Handling    | transparency control   |

---

## Internal Components

```text id="overlay-components"
overlay/
├── renderer.rs
├── capsule.rs
├── transcript_ticker.rs
├── ai_renderer.rs
├── transparency.rs
├── blur.rs
├── animations.rs
├── positioning.rs
├── hotkeys.rs
└── tray.rs
```

---

## Inputs

Receives:

* transcript events
* AI token streams
* session state
* user interactions

---

## Outputs

Produces:

* overlay events
* interaction events
* help trigger events

---

## Major Technical Challenges

| Area                | Complexity |
| ------------------- | ---------- |
| Non-focus rendering | High       |
| Alt+Tab exclusion   | High       |
| Transparency        | Medium     |
| GPU acceleration    | Medium     |

---

# 2. Audio Module

## Purpose

Captures:

* microphone audio
* system audio

using:

```text id="audio-api"
WASAPI
```

---

## Responsibilities

| Responsibility         | Description            |
| ---------------------- | ---------------------- |
| Mic Capture            | default device capture |
| Loopback Capture       | system audio           |
| Device Recovery        | reconnect handling     |
| Audio Buffering        | stream chunking        |
| Stream Synchronization | source timing          |
| Audio Monitoring       | device state           |

---

## Internal Components

```text id="audio-components"
audio/
├── mic_capture.rs
├── system_capture.rs
├── device_manager.rs
├── audio_buffer.rs
├── synchronization.rs
└── audio_events.rs
```

---

## Inputs

Receives:

* Windows audio devices
* system device changes

---

## Outputs

Produces:

* audio chunks
* audio events
* source metadata

---

# 3. STT Module

## Purpose

Real-time speech transcription.

Uses:

```text id="stt-engine"
Whisper.cpp
```

---

## Responsibilities

| Responsibility          | Description         |
| ----------------------- | ------------------- |
| Streaming Transcription | live inference      |
| Chunk Processing        | audio segmentation  |
| Language Optimization   | English-only        |
| Transcript Cleanup      | punctuation cleanup |
| Source Labeling         | user/speaker        |
| Transcript Streaming    | overlay updates     |

---

## Internal Components

```text id="stt-components"
stt/
├── whisper.rs
├── streaming.rs
├── chunking.rs
├── transcript_cleaner.rs
├── labeling.rs
└── inference_queue.rs
```

---

## Inputs

Receives:

* audio chunks
* source labels

---

## Outputs

Produces:

* transcript chunks
* transcript events

---

## Performance Targets

| Metric    | Target      |
| --------- | ----------- |
| Latency   | <1 second   |
| CPU Usage | low         |
| RAM Usage | lightweight |

---

# 4. AI Module

## Purpose

Handles:

* AI provider integration
* context extraction
* prompt building
* streaming responses

---

## Responsibilities

| Responsibility       | Description                 |
| -------------------- | --------------------------- |
| Context Extraction   | recent transcript selection |
| Prompt Construction  | AI request building         |
| Provider Abstraction | multiple AI APIs            |
| Streaming Handling   | token streams               |
| Long-Press Logic     | extended context            |
| Response Formatting  | professional output         |

---

## Internal Components

```text id="ai-components"
ai/
├── providers/
├── prompt_builder.rs
├── context_extractor.rs
├── request_builder.rs
├── streaming.rs
├── hold_context.rs
└── ai_events.rs
```

---

## Inputs

Receives:

* transcript history
* help trigger events
* system prompts

---

## Outputs

Produces:

* AI token streams
* AI response events

---

# 5. Persistence Module

## Purpose

Handles all local storage.

Uses:

```text id="db-choice"
SQLite
```

---

## Responsibilities

| Responsibility     | Description            |
| ------------------ | ---------------------- |
| Session Storage    | meeting persistence    |
| Transcript Storage | live transcript writes |
| AI Log Storage     | response history       |
| Querying           | history retrieval      |
| Export Data Access | export preparation     |

---

## Internal Components

```text id="persistence-components"
persistence/
├── db.rs
├── sessions.rs
├── transcripts.rs
├── ai_logs.rs
├── migrations/
└── indexing.rs
```

---

## Inputs

Receives:

* transcript chunks
* AI responses
* session metadata

---

## Outputs

Produces:

* stored records
* history retrieval

---

# 6. Session Module

## Purpose

Manages:

* active session state
* meeting metadata
* session lifecycle

---

## Responsibilities

| Responsibility      | Description             |
| ------------------- | ----------------------- |
| Session Creation    | automatic startup       |
| Session Metadata    | title/prompt            |
| Active State        | current session         |
| Session Closing     | shutdown handling       |
| Session Restoration | previous state recovery |

---

## Internal Components

```text id="session-components"
session/
├── manager.rs
├── state.rs
├── metadata.rs
├── lifecycle.rs
└── session_events.rs
```

---

# 7. Export Module

## Purpose

Exports:

* transcripts
* AI responses
* metadata

---

## Responsibilities

| Responsibility    | Description       |
| ----------------- | ----------------- |
| TXT Export        | raw transcript    |
| Markdown Export   | structured output |
| Future PDF Export | printable reports |

---

## Internal Components

```text id="export-components"
export/
├── markdown.rs
├── txt.rs
├── formatter.rs
└── export_events.rs
```

---

# 8. System Integration Module

## Purpose

Handles Windows-native integration.

---

## Responsibilities

| Responsibility       | Description      |
| -------------------- | ---------------- |
| Tray Integration     | system tray      |
| Startup Registration | launch behavior  |
| Alt+Tab Exclusion    | overlay hiding   |
| Window Flags         | overlay behavior |
| Hotkeys              | global shortcuts |

---

## Internal Components

```text id="system-components"
system/
├── windows/
│   ├── focus.rs
│   ├── startup.rs
│   ├── tray.rs
│   ├── alt_tab.rs
│   ├── hotkeys.rs
│   └── window_flags.rs
```

---

# 9. Config Module

## Purpose

Stores:

* user settings
* API keys
* preferences

---

## Responsibilities

| Responsibility   | Description       |
| ---------------- | ----------------- |
| Provider Config  | API providers     |
| API Key Storage  | local credentials |
| Overlay Settings | opacity/theme     |
| Startup Settings | launch behavior   |
| Prompt Settings  | user prompts      |

---

## Internal Components

```text id="config-components"
config/
├── settings.rs
├── providers.rs
├── prompts.rs
├── persistence.rs
└── defaults.rs
```

---

# 10. Event Bus Module

## Purpose

Internal async communication layer.

Architecture:

```text id="event-pattern"
Pub/Sub Event Bus
```

---

## Responsibilities

| Responsibility       | Description         |
| -------------------- | ------------------- |
| Event Dispatch       | async routing       |
| Module Communication | decoupling          |
| State Updates        | runtime propagation |

---

## Internal Components

```text id="event-components"
events/
├── bus.rs
├── transcript_events.rs
├── ai_events.rs
├── overlay_events.rs
├── audio_events.rs
└── system_events.rs
```

---

# Runtime Communication Flow

```text id="communication-flow"
Audio Module
    ↓
STT Module
    ↓
Persistence Module
    ↓
Overlay Module
    ↓
User presses Help Me
    ↓
AI Module
    ↓
Overlay Module
```

---

# Concurrency Model

## Runtime

```text id="runtime-choice"
Tokio Async Runtime
```

---

## Parallel Systems

| Module            | Parallel Execution |
| ----------------- | ------------------ |
| Audio Capture     | Yes                |
| STT Inference     | Yes                |
| AI Streaming      | Yes                |
| Overlay Rendering | Yes                |
| Database Writes   | Yes                |

---

# Module Isolation Principles

Each module must:

* expose clean interfaces
* avoid direct cross-module state mutation
* communicate via events
* remain independently testable

---

# Future Module Expansion

## Planned Future Modules

### OCR Module

```text id="future-ocr"
screen analysis + OCR
```

### Search Module

```text id="future-search"
transcript indexing/search
```

### Linux Module

```text id="future-linux"
Wayland/X11 integration
```

---

# Final Engineering Principle

```text id="engineering-principle"
Native-first
Low-latency
Overlay-centric
Event-driven
Modular monolith
```

This module structure is approved for:

* implementation planning
* engineering ticket generation
* repository scaffolding
* prototype development.
