# Akela — Fresh Start Roadmap

## 🎯 Phase 1: Core Audio Engine (High Stability)
- [ ] Implement a clean WASAPI-based Microphone capture loop.
- [ ] Implement a clean WASAPI-based System Loopback capture (speaker audio).
- [ ] Develop a `SynchronizationBuffer` to merge Mic and System audio into a single stream.
- [ ] Ensure the engine can handle sample rate mismatches (e.g., 44.1kHz vs 48kHz).

## 🎙️ Phase 2: Streaming STT (Local First)
- [ ] Integrate `whisper-rs` (binding for Whisper.cpp).
- [ ] Download and verify the `base.en` or `small.en` model.
- [ ] Implement a sliding window buffer for real-time transcription.
- [ ] Expose live transcripts to the frontend via Tauri Events.

## 🐺 Phase 3: AI Context & Chat
- [ ] Implement "Help Me" trigger logic.
- [ ] Build the context window (extracting the last X minutes of transcripts).
- [ ] Integrate streaming AI responses (OpenAI/Gemini).

## 🎨 Phase 4: UI & Polish
- [ ] Expandable Chat Panel in the overlay.
- [ ] Transcription Ticker with auto-scroll and manual pause.
- [ ] Settings panel for Model selection and API keys.

---
*Last Updated: 2026-05-10*
