# 🐺 Akela

> Local-first overlay AI assistant for meetings, research, and real-time contextual help.

<p align="center">
  <img src="./assets/images/akela-banner.png" width="100%" alt="Akela Banner" />
</p>

<p align="center">
  <img src="https://img.shields.io/badge/platform-Windows-blue" />
  <img src="https://img.shields.io/badge/status-Active%20Development-orange" />
  <img src="https://img.shields.io/badge/license-MIT-green" />
  <img src="https://img.shields.io/badge/frontend-React%20%2B%20Tauri-blueviolet" />
  <img src="https://img.shields.io/badge/backend-Rust-orange" />
</p>

---

# ⚠️ Project Status

Akela is currently under active development.

This repository is in:
- architecture phase
- overlay prototyping phase
- audio pipeline experimentation phase

The project is **not production ready yet**.

Major systems currently being developed:
- Native Windows overlay renderer
- Real-time transcription pipeline
- WASAPI audio capture
- AI streaming integration
- Overlay interaction system

---

# Overview

Akela is a lightweight Windows overlay assistant designed for:
- meetings
- research discussions
- technical conversations
- note assistance
- contextual AI help

Unlike traditional AI chat applications, Akela works as a:

```text
Passive Overlay AI Layer
```

The application continuously:
- captures meeting audio locally
- transcribes speech in real time
- stores transcripts locally
- allows users to trigger contextual AI responses instantly

All heavy AI reasoning is cloud-based using the user's own API keys.

---

# Core Philosophy

Akela is built around these principles:

| Principle | Description |
|---|---|
| Local-first | transcripts remain on device |
| Open-source | MIT licensed |
| Lightweight | optimized for normal laptops |
| Overlay-native | minimal workflow interruption |
| User-controlled | manual AI triggering |
| No backend | no Akela servers |
| Privacy-oriented | user-owned data and keys |

---

# Features

## Current MVP Scope

### Overlay System
- Floating capsule overlay
- Always-on-top rendering
- Transparent glassmorphism UI
- Expandable overlay panel
- Hidden from taskbar
- Hidden from Alt+Tab
- Non-focus stealing behavior

---

### Real-Time Transcription
- Live speech-to-text
- Microphone capture
- System audio capture
- Continuous transcript ticker
- Local transcript persistence

---

### AI Assistance
- "Help Me" contextual trigger
- Streaming AI responses
- Hold-to-capture extended context
- Custom system prompts
- Multi-provider support

---

### Persistence
- SQLite-based local storage
- Session history
- Transcript archive
- Export system

---

### Export System
- TXT export
- Markdown export
- Session metadata export

---

# Architecture

## High-Level Architecture

```text
+--------------------------------------------------+
|                    Akela                         |
+--------------------------------------------------+
|                                                  |
|  Tauri Shell                                     |
|  ├── React Frontend                              |
|  ├── Settings UI                                 |
|  ├── History UI                                  |
|  └── Tray Integration                            |
|                                                  |
|  Native Overlay Engine                           |
|  ├── Capsule Renderer                            |
|  ├── Transparency                                |
|  ├── Blur Effects                                |
|  ├── AI Stream Renderer                          |
|  └── Transcript Ticker                           |
|                                                  |
|  Audio Engine                                    |
|  ├── Microphone Capture                          |
|  ├── WASAPI Loopback                             |
|  └── Audio Routing                               |
|                                                  |
|  STT Engine                                      |
|  ├── Whisper.cpp                                 |
|  ├── Streaming Transcription                     |
|  └── Transcript Processing                       |
|                                                  |
|  AI Engine                                       |
|  ├── OpenAI                                      |
|  ├── Gemini                                      |
|  ├── NVIDIA APIs                                 |
|  └── Streaming Responses                         |
|                                                  |
|  Persistence Engine                              |
|  ├── SQLite                                      |
|  ├── Session Storage                             |
|  └── Transcript Storage                          |
|                                                  |
+--------------------------------------------------+
```

---

# Tech Stack

| Layer | Technology |
|---|---|
| Language | Rust |
| Shell | Tauri v2 |
| Frontend | React + TypeScript |
| Styling | TailwindCSS |
| Runtime | Tokio |
| Database | SQLite |
| STT | Whisper.cpp |
| Audio | WASAPI |
| Overlay | Win32 + DirectComposition |
| Package Manager | Bun |

---

# Why Akela Exists

Most AI tools require:
- switching tabs
- manually copying context
- interrupting workflow

Akela is designed to reduce friction by:
- staying visible as an overlay
- understanding recent conversation context
- providing instant contextual assistance

The goal is:
```text
minimal interruption, immediate contextual help
```

---

# Current Development Priorities

## Phase 1 — Overlay Engine
- Transparent capsule overlay
- Native Win32 rendering
- Blur/transparency
- Overlay movement
- Alt+Tab exclusion

---

## Phase 2 — Audio Pipeline
- WASAPI microphone capture
- WASAPI loopback capture
- Audio synchronization
- Source separation

---

## Phase 3 — Streaming STT
- Whisper integration
- Low-latency transcription
- Transcript rendering

---

## Phase 4 — AI Integration
- Streaming providers
- Context extraction
- Prompt system

---

# Repository Structure

```text
akela/
│
├── frontend/
├── src-tauri/
├── native-overlay/
├── docs/
├── assets/
├── models/
├── tests/
└── scripts/
```

---

# Development Setup

## Requirements

### Windows
- Windows 10/11

### Rust
Install:
https://www.rust-lang.org/tools/install

### Bun
Install:
https://bun.sh/

### Visual Studio Build Tools
Install:
- Desktop development with C++

---

# Local Development

## Clone Repository

```bash
git clone https://github.com/your-org/akela.git
cd akela
```

## 🚀 Running the Project

### Development Mode (Dev)

Use this mode while actively coding. It spins up the Vite frontend development server and the Tauri Rust backend, enabling Hot Module Replacement (HMR) for the UI.

```bash
# Run the entire app in dev mode
bun tauri dev
```

If you only want to work on the UI in a standard web browser without the native desktop shell:
```bash
bun run dev
```
*(Note: Native features like transparent overlays and audio capture will not work in the browser).*

### Production Build (Actual/Release)

To compile the application into a standalone Windows executable (`.exe`) and MSI installer, run:

```bash
bun tauri build
```

The compiled binaries will be located in `src-tauri/target/release/`.
- `akela.exe`: The standalone executable.
- `bundle/msi/`: The Windows installer package.

---

# Current MVP Goals

The first public MVP aims to provide:

- stable overlay rendering
- real-time transcription
- contextual AI responses
- local session persistence
- lightweight performance

NOT:
- OCR
- Linux support
- advanced semantic search
- multi-session workflows

Those are planned for later versions.

---

# Planned Roadmap

## V1
- Overlay engine
- Real-time transcription
- AI streaming
- Session persistence

---

## V1.1
- Better transcript search
- Improved animations
- Smarter context extraction

---

## V1.2
- OCR/screen analysis
- Process-aware audio filtering

---

## V2
- Linux support
- Semantic search
- Advanced meeting intelligence

---

# Performance Targets

| Metric | Target |
|---|---|
| RAM Usage | ~1.3GB max |
| STT Delay | <1 second |
| AI First Token | <2 seconds |
| Overlay FPS | 60 FPS target |

---

# Security & Privacy

Akela:
- does NOT use Akela-owned servers
- does NOT require accounts
- does NOT upload transcripts automatically
- does NOT store API keys remotely

Users provide their own:
- OpenAI keys
- Gemini keys
- NVIDIA API keys

All transcripts remain local.

---

# Open Source

Licensed under:
```text
MIT License
```

Contributions are welcome.

---

# Contributing

Current areas where contributions are highly valuable:

- Win32 overlay rendering
- WASAPI audio capture
- Whisper optimization
- Overlay animations
- Tauri integration
- Performance profiling

---

# Current Challenges

Akela is solving several difficult engineering problems simultaneously:

- Non-focus overlay rendering
- Transparent GPU overlays
- WASAPI dual capture
- Low-latency STT
- Real-time AI streaming
- Lightweight desktop performance

---

# Screenshots

> Placeholder screenshots — UI currently under development.

---

# Disclaimer

Akela is a productivity and contextual assistance tool.

Users are responsible for complying with:
- workplace policies
- institutional policies
- applicable laws
- platform terms of service

---

# License

MIT License

---

# Development Status

```text
Overlay Prototype Phase
```

Core systems currently under active implementation:
- overlay renderer
- WASAPI integration
- Whisper streaming
- AI provider abstraction

---

# Acknowledgements

Inspired by:
- overlay UX systems
- real-time transcription tools
- contextual AI workflows
- Akela from *The Jungle Book*

---

# Future Vision

Akela aims to become:

```text
A lightweight contextual intelligence layer for desktop workflows.
```

# Akela 🐺

> A local-first Windows overlay AI meeting assistant designed to be intelligent, observant, and non-intrusive.

Akela continuously transcribes meeting audio locally, stores transcripts, and provides contextual AI assistance through a floating overlay capsule that is completely invisible to screen capturing tools (OBS, Zoom, screenshots, etc.).

---

## 🛠️ Prerequisites

Before you can run or build Akela, ensure your development environment has the following installed:

1. **Rust Toolchain**: [Install Rust](https://www.rust-lang.org/tools/install) (includes `cargo`).
2. **Bun**: [Install Bun](https://bun.sh/docs/installation) (used as our frontend runtime and package manager).
3. **Visual Studio C++ Build Tools**: Required for compiling native Windows APIs and Tauri.
   - Download [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).
   - Select **Desktop development with C++** during installation.

---

## ⚙️ Environment Setup

1. Clone the repository and navigate to the project root.
2. Install the frontend dependencies using Bun:
   ```bash
   bun install
   ```
3. Set up your environment variables:
   - Copy `.env.example` to `.env`
   - Add your preferred AI API keys (OpenAI, Gemini, or NVIDIA).

---

---

## 🏗️ Project Architecture

Akela uses a **Modular Monolith** architecture:
- **Frontend**: React, TypeScript, TailwindCSS, Zustand, Framer Motion (managed by Vite).
- **Backend**: Rust, Tauri v2, Tokio (Async Runtime).
- **Storage**: SQLite (via `sqlx`).
- **AI/STT**: Whisper.cpp (via `whisper-rs`), OpenAI/Gemini APIs.
- **Native Windows**: Win32 APIs (WASAPI for audio, DirectComposition for overlay, SetWindowDisplayAffinity for screen-capture invisibility).

---

## 📜 License
MIT License