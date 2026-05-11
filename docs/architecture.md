# System Architecture

Akela is built as a high-performance Windows desktop application using **Tauri v2**, **Rust**, and **React**. It is designed to be "invisible" until needed, operating as a system-level overlay.

---

## High-Level Diagram

```mermaid
graph TD
    subgraph Frontend (React + TypeScript)
        UI[Capsule Overlay UI]
        Store[Zustand State Management]
    end

    subgraph Backend (Rust + Tauri)
        Engine[Audio Orchestrator]
        Capture[WASAPI Capture threads]
        VAD[WebRTC VAD Pipeline]
        STT[Whisper.cpp Inference]
        Win32[Native Window Management]
    end

    UI <--> Store
    Store <--> Tauri_Events[Tauri Event Bridge]
    Tauri_Events <--> Engine
    Engine --> Capture
    Capture --> VAD
    VAD --> STT
    STT --> Tauri_Events
    Win32 --> UI
```

---

## Core Components

### 1. The Overlay System
The UI is a transparent, frameless window managed via native Win32 API calls (`SetWindowDisplayAffinity`, `SetWindowPos`, etc.).
- **Stealth Mode**: The app is hidden from the taskbar and Alt+Tab menu.
- **Zero-Focus**: It remains always-on-top but does not steal focus from other apps.

### 2. The Audio Engine
A dual-pipeline system that captures audio simultaneously from two sources:
- **Microphone**: Direct user input via CPAL/WASAPI.
- **System Loopback**: Captures "what you hear" (meeting audio) using WASAPI Loopback.

### 3. The STT Pipeline (Speech-to-Text)
Audio is processed through a multi-stage pipeline:
- **Scout VAD**: Fast silence detection using Google WebRTC VAD.
- **Pre-Roll Buffer**: Ensures the start of sentences isn't clipped.
- **Whisper Inference**: Local-first transcription using the `whisper-rs` bindings.

### 4. State Management
- **Frontend**: Zustand manages the UI state, transcript history, and live ticker.
- **Backend**: Managed state within Tauri handles the singleton `AudioEngine` lifecycle and cross-thread communication.

---

## Tech Stack
- **Frontend**: React, TypeScript, Vite, Tailwind CSS (optional), Lucide Icons, Zustand.
- **Backend**: Rust, Tauri v2.
- **Audio**: `cpal`, `wasapi-rs`, `rubato` (resampling), `ringbuf`.
- **AI/ML**: `whisper-rs` (Whisper.cpp bindings).
- **Native**: `windows-rs` for deep OS integration.
