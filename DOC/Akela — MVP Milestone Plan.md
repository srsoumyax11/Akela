# Akela — MVP Milestone Plan

## MVP Goal

Deliver a:

```text id="mvp-goal"
Working Windows overlay AI assistant
```

with:

* live transcription
* floating capsule overlay
* Help Me AI interaction
* local transcript persistence
* streaming AI responses

---

# MVP Development Philosophy

## Priorities

```text id="mvp-priority"
Functionality first
Overlay stability second
Visual polish third
```

Reason:

* overlay rendering is technically difficult
* audio synchronization is critical
* latency determines usability

---

# Estimated MVP Timeline

| Phase                  | Duration  |
| ---------------------- | --------- |
| Foundation             | 1 week    |
| Overlay Prototype      | 1–2 weeks |
| Audio + STT            | 1–2 weeks |
| AI Integration         | 1 week    |
| Persistence            | 3–5 days  |
| Overlay UX Integration | 1 week    |
| Testing/Stabilization  | 1 week    |

Approximate:

```text id="timeline-total"
6–8 weeks MVP
```

for a focused small team.

---

# Milestone 0 — Repository & Foundation

## Goal

Initialize development environment.

---

## Deliverables

### Repository Setup

```text id="m0-repo"
- monorepo initialized
- frontend scaffolded
- src-tauri scaffolded
- cargo workspace configured
```

---

### Tooling

```text id="m0-tooling"
- Rust toolchain
- pnpm
- Tauri
- linting
- formatting
- CI pipeline
```

---

### Build Validation

```text id="m0-build"
- app launches
- Tauri shell works
- Windows build succeeds
```

---

# Milestone 1 — Native Overlay Prototype

## Highest Priority Milestone

This validates the hardest technical problem first.

---

# Goals

Validate:

* overlay rendering
* transparency
* focus behavior
* Alt+Tab exclusion
* always-on-top behavior

---

## Deliverables

### Floating Capsule Overlay

```text id="m1-overlay"
- frameless window
- rounded capsule
- transparency
- movable
```

---

### Window Behavior

```text id="m1-window"
- hidden from taskbar
- hidden from Alt+Tab
- always-on-top
- non-focus stealing
```

---

### Overlay States

```text id="m1-states"
- compact mode
- expanded mode
- collapse animations
```

---

## Success Criteria

| Requirement       | Status   |
| ----------------- | -------- |
| Overlay visible   | Required |
| Smooth rendering  | Required |
| No taskbar icon   | Required |
| No focus stealing | Required |
| Stable movement   | Required |

---

# Milestone 2 — Audio Capture System

## Goal

Implement dual-source audio capture.

---

## Deliverables

### Microphone Capture

```text id="m2-mic"
WASAPI microphone capture
```

---

### System Audio Capture

```text id="m2-system"
WASAPI loopback capture
```

---

### Audio Routing

```text id="m2-routing"
- source labeling
- synchronization
- buffering
```

---

### Device Handling

```text id="m2-devices"
- default device detection
- reconnect recovery
```

---

## Success Criteria

| Requirement        | Status   |
| ------------------ | -------- |
| Mic capture stable | Required |
| Loopback stable    | Required |
| Minimal latency    | Required |
| Low CPU usage      | Required |

---

# Milestone 3 — STT Integration

## Goal

Real-time transcription pipeline.

---

## Deliverables

### Whisper Integration

```text id="m3-whisper"
Whisper.cpp integrated
```

---

### Streaming Pipeline

```text id="m3-streaming"
- chunk processing
- transcript updates
- transcript cleanup
```

---

### Overlay Transcript Ticker

```text id="m3-ticker"
live scrolling transcript
```

---

## Success Criteria

| Requirement           | Status   |
| --------------------- | -------- |
| <1 sec delay          | Required |
| Stable streaming      | Required |
| English transcription | Required |
| Low memory usage      | Required |

---

# Milestone 4 — Persistence Layer

## Goal

Store everything locally.

---

## Deliverables

### SQLite Integration

```text id="m4-sqlite"
- session storage
- transcript storage
- AI logs
```

---

### Session Lifecycle

```text id="m4-session"
- automatic session creation
- automatic persistence
```

---

### History Retrieval

```text id="m4-history"
retrieve previous sessions
```

---

## Success Criteria

| Requirement             | Status   |
| ----------------------- | -------- |
| Data persists correctly | Required |
| Sessions recoverable    | Required |
| No corruption           | Required |

---

# Milestone 5 — AI Integration

## Goal

Enable contextual AI assistance.

---

## Deliverables

### Provider System

```text id="m5-providers"
- OpenAI
- Gemini
- NVIDIA
```

---

### Streaming Responses

```text id="m5-streaming"
token-by-token rendering
```

---

### Help Me Button

```text id="m5-help"
- click mode
- hold mode
```

---

### Prompt Builder

```text id="m5-prompts"
- transcript extraction
- system prompts
- provider abstraction
```

---

## Success Criteria

| Requirement              | Status   |
| ------------------------ | -------- |
| AI responses stream      | Required |
| Context extraction works | Required |
| Hold behavior works      | Required |

---

# Milestone 6 — Overlay UX Integration

## Goal

Merge all systems into final overlay experience.

---

## Deliverables

### Compact Capsule

```text id="m6-capsule"
final compact overlay
```

---

### Expanded Overlay

```text id="m6-expanded"
history/settings panel
```

---

### Settings System

```text id="m6-settings"
- API keys
- opacity
- startup behavior
```

---

### Tray Integration

```text id="m6-tray"
system tray support
```

---

## Success Criteria

| Requirement    | Status   |
| -------------- | -------- |
| Smooth UX      | Required |
| No crashes     | Required |
| Overlay stable | Required |

---

# Milestone 7 — Export System

## Goal

Allow transcript exporting.

---

## Deliverables

### TXT Export

```text id="m7-txt"
raw transcript export
```

---

### Markdown Export

```text id="m7-md"
structured meeting export
```

---

## Success Criteria

| Requirement        | Status   |
| ------------------ | -------- |
| Export accurate    | Required |
| Metadata preserved | Required |

---

# Milestone 8 — Stabilization & QA

## Goal

Prepare usable MVP release.

---

## Deliverables

### Performance Optimization

```text id="m8-performance"
- CPU optimization
- memory reduction
- async tuning
```

---

### Stability Testing

```text id="m8-testing"
- long meeting tests
- audio reconnect tests
- overlay stress tests
```

---

### Packaging

```text id="m8-package"
- MSI installer
- EXE build
- GitHub releases
```

---

## Success Criteria

| Requirement              | Status   |
| ------------------------ | -------- |
| Stable for long sessions | Required |
| Installer works          | Required |
| Memory stable            | Required |

---

# MVP Completion Definition

Akela MVP is complete when:

```text id="mvp-definition"
User launches app
    ↓
Overlay appears
    ↓
Transcription begins automatically
    ↓
User presses Help Me
    ↓
AI response streams live
    ↓
Session persists locally
```

with:

* stable overlay behavior
* acceptable latency
* usable meeting experience

---

# Post-MVP Roadmap

## V1.1

* transcript search
* better overlay animations
* smarter context extraction

---

## V1.2

* OCR/screen analysis
* process-aware audio filtering

---

## V2

* Linux support
* advanced AI memory
* semantic search

---

# Critical MVP Engineering Risks

| Risk                    | Priority |
| ----------------------- | -------- |
| Overlay focus bugs      | Critical |
| WASAPI instability      | Critical |
| STT latency spikes      | High     |
| Transparency rendering  | High     |
| AI stream interruptions | Medium   |

---

# Recommended MVP Team Focus

## Priority Order

```text id="team-focus"
1. Overlay
2. Audio
3. STT
4. AI streaming
5. Persistence
6. UX polish
```

Because:

* overlay failure kills product usability
* audio failure kills core functionality

---

# Final MVP Philosophy

```text id="mvp-philosophy"
Stable
Fast
Minimal
Overlay-native
Local-first
```

This milestone plan is approved for:

* engineering execution
* sprint planning
* task breakdown
* prototype implementation
* MVP delivery tracking.
