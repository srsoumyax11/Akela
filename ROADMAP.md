# Akela Product Roadmap

This document outlines the development path for Akela, from its current state to our long-term vision for a privacy-first, zero-friction meeting assistant.

---

## Phase 1: Foundation (Completed)
- [x] Project architecture with Rust, Tauri, and React.
- [x] High-performance, transparent overlay system using native Win32 APIs.
- [x] Zero-focus interaction model (app stays out of Alt+Tab and taskbar).
- [x] Multi-monitor support and position persistence.

## Phase 2: Dual-Stream Audio Engine (Current)
- [x] WASAPI Microphone capture pipeline.
- [x] WASAPI System Loopback (capture meeting audio).
- [x] Integrated Whisper STT engine (local-first transcription).
- [/] **In Progress**: Real-time transcript ticker in the capsule.
- [ ] Adaptive Voice Activity Detection (VAD) optimization.
- [ ] Audio device recovery (Bluetooth/USB hot-swapping).

## Phase 3: AI Intelligence & Extraction (Upcoming)
- [ ] Streaming AI responses (OpenAI, Gemini, NVIDIA NIM).
- [ ] Contextual extraction (one-click to analyze last ~50 words).
- [ ] "Hold-to-Listen" mode for continuous context accumulation.
- [ ] Streaming word-by-word animations in the overlay.

## Phase 4: Persistence & Knowledge Base
- [ ] Local SQLite storage for all meeting sessions.
- [ ] Session history browser and search.
- [ ] Privacy-first local knowledge base integration.
- [ ] Export to Markdown and TXT formats.

## Phase 5: Polish & Ecosystem
- [ ] Global hotkey management (Ctrl+Shift+A).
- [ ] System tray integration.
- [ ] Automated Windows packaging (MSI/EXE).
- [ ] Support for local LLMs (Llama.cpp integration).

---

## Long-Term Vision
Our goal is to make Akela the "invisible layer" for all digital communication—providing instant, AI-powered insights without the user ever having to switch windows or manage complex recording tools.
