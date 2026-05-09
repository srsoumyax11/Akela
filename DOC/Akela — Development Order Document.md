# Akela — Development Order Document

## Development Philosophy

Akela should NOT be built feature-first.

It should be built:

```text id="dev-philosophy"
Risk-first
```

Meaning:

* hardest technical systems validated first
* architecture stabilized early
* UI polish delayed until infrastructure works

Because:

* overlay rendering
* audio capture
* non-focus interaction

are the real technical risks.

---

# Core Development Strategy

## Build Order Priority

```text id="priority-order"
1. Overlay foundation
2. Audio capture
3. STT pipeline
4. AI streaming
5. Persistence
6. UX integration
7. Export/features
8. Optimization/polish
```

---

# Phase 0 — Environment & Repository Setup

## Goal

Create stable engineering foundation.

---

## Tasks

### Repository Initialization

```text id="phase0-repo"
- create monorepo
- configure Cargo workspace
- initialize frontend
- initialize Tauri
```

---

### Tooling

```text id="phase0-tools"
- Rust toolchain
- pnpm
- clippy
- rustfmt
- eslint
- prettier
```

---

### CI Setup

```text id="phase0-ci"
- Windows build workflow
- lint checks
- release pipeline
```

---

# Deliverable

```text id="phase0-output"
Application launches successfully
```

---

# Phase 1 — Native Overlay Foundation

# Highest Priority Phase

This phase determines whether the project architecture is viable.

---

# Goal

Validate:

* overlay rendering
* transparency
* blur
* movement
* Alt+Tab exclusion
* focus behavior

---

## Tasks

### Create Frameless Overlay

```text id="phase1-window"
- transparent window
- rounded capsule
- always-on-top
```

---

### Implement Window Flags

```text id="phase1-flags"
- no taskbar icon
- Alt+Tab exclusion
- non-focus window
```

---

### Overlay Positioning

```text id="phase1-position"
- draggable overlay
- position persistence
```

---

### Expand/Collapse States

```text id="phase1-states"
- compact mode
- expanded mode
- animation transitions
```

---

# Critical Validation

If THIS phase fails:

* entire architecture changes

So this must come first.

---

# Deliverable

```text id="phase1-output"
Stable floating capsule overlay
```

---

# Phase 2 — Audio Engine

## Goal

Reliable dual-source audio capture.

---

## Tasks

### WASAPI Microphone Capture

```text id="phase2-mic"
Capture default microphone
```

---

### WASAPI Loopback

```text id="phase2-loopback"
Capture system audio
```

---

### Source Classification

```text id="phase2-source"
- user source
- speaker source
```

---

### Audio Buffering

```text id="phase2-buffer"
- chunk queues
- synchronization
- buffering
```

---

### Device Recovery

```text id="phase2-recovery"
- reconnect handling
- device switching
```

---

# Deliverable

```text id="phase2-output"
Continuous low-latency audio streams
```

---

# Phase 3 — STT Streaming Pipeline

## Goal

Real-time transcription.

---

## Tasks

### Whisper.cpp Integration

```text id="phase3-whisper"
Integrate whisper.cpp
```

---

### Streaming Inference

```text id="phase3-streaming"
- chunk processing
- transcript streaming
```

---

### Transcript Cleanup

```text id="phase3-cleanup"
- punctuation cleanup
- transcript normalization
```

---

### Live Transcript Overlay

```text id="phase3-overlay"
Scrolling transcript ticker
```

---

# Deliverable

```text id="phase3-output"
Stable live transcription
```

---

# Phase 4 — Persistence Layer

## Goal

Store all runtime data safely.

---

## Tasks

### SQLite Integration

```text id="phase4-db"
Initialize SQLite
```

---

### Session Management

```text id="phase4-session"
- create sessions
- persist sessions
```

---

### Transcript Persistence

```text id="phase4-transcript"
- async transcript writes
- chunk storage
```

---

### History Retrieval

```text id="phase4-history"
Retrieve previous sessions
```

---

# Deliverable

```text id="phase4-output"
Persistent transcript history
```

---

# Phase 5 — AI Engine

## Goal

Contextual AI generation.

---

## Tasks

### Provider Abstraction

```text id="phase5-provider"
- OpenAI
- Gemini
- NVIDIA
```

---

### Streaming Responses

```text id="phase5-stream"
token streaming
```

---

### Context Extraction

```text id="phase5-context"
- recent transcript extraction
- hold accumulation mode
```

---

### Help Me Interaction

```text id="phase5-help"
- click mode
- hold mode
```

---

# Deliverable

```text id="phase5-output"
Live contextual AI responses
```

---

# Phase 6 — Overlay UX Integration

## Goal

Merge all systems into cohesive UX.

---

## Tasks

### Compact Capsule UI

```text id="phase6-capsule"
Final compact overlay
```

---

### Expanded Overlay

```text id="phase6-expanded"
Settings/history panel
```

---

### Settings Integration

```text id="phase6-settings"
- API keys
- opacity
- startup behavior
```

---

### Tray Integration

```text id="phase6-tray"
System tray behavior
```

---

# Deliverable

```text id="phase6-output"
Fully integrated overlay experience
```

---

# Phase 7 — Export System

## Goal

Allow transcript export.

---

## Tasks

### TXT Export

```text id="phase7-txt"
Raw transcript export
```

---

### Markdown Export

```text id="phase7-md"
Structured export
```

---

### Export UI

```text id="phase7-ui"
Export controls
```

---

# Deliverable

```text id="phase7-output"
Usable export system
```

---

# Phase 8 — Optimization & Stabilization

## Goal

Prepare public MVP.

---

## Tasks

### Memory Optimization

```text id="phase8-memory"
Reduce RAM usage
```

---

### Async Optimization

```text id="phase8-async"
Improve concurrency
```

---

### Overlay Optimization

```text id="phase8-overlay"
Improve rendering smoothness
```

---

### Stress Testing

```text id="phase8-stress"
Long meeting tests
```

---

### Packaging

```text id="phase8-package"
- MSI installer
- EXE release
```

---

# Deliverable

```text id="phase8-output"
Stable public MVP release
```

---

# Recommended Team Parallelization

## Can Run Concurrently

| Team          | Phase        |
| ------------- | ------------ |
| Overlay Team  | Phase 1      |
| Audio Team    | Phase 2      |
| Frontend Team | Phase 6 prep |
| Backend Team  | Phase 4 prep |

---

# Cannot Be Fully Parallelized

| Dependency                         | Reason               |
| ---------------------------------- | -------------------- |
| AI before STT                      | no transcript source |
| Overlay polish before overlay core | unstable foundation  |
| Export before persistence          | no stored data       |

---

# Recommended Git Workflow

## Branching

```text id="git-workflow"
main
develop
feature/*
```

---

# Recommended Commit Strategy

```text id="commit-strategy"
feat:
fix:
refactor:
perf:
docs:
```

---

# Recommended MVP Freeze Point

Freeze MVP after:

```text id="freeze-point"
Phase 6 complete
```

Then:

* stabilize
* optimize
* release

Do NOT endlessly add features before stabilization.

---

# Engineering Priority Matrix

| System        | Priority |
| ------------- | -------- |
| Overlay       | Critical |
| Audio         | Critical |
| STT           | Critical |
| AI            | High     |
| Persistence   | Medium   |
| Export        | Low      |
| Visual Polish | Low      |

---

# Final Development Principle

```text id="development-principle"
Build the hardest systems first.
Polish last.
```

This development order is approved for:

* sprint planning
* engineering assignment
* implementation sequencing
* milestone tracking
* MVP execution.
