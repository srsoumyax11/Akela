# Akela — Engineering Responsibilities Document

## Team Structure Overview

Akela is architected as a:

```text id="team-model"
Native-first overlay desktop application
```

This requires:

* systems engineering
* real-time audio processing
* native Windows integration
* frontend UX engineering
* AI pipeline engineering

The responsibilities below are organized by technical ownership.

---

# 1. Technical Lead / Architect

## Role

Owns:

* overall architecture
* technical decisions
* integration strategy
* milestone planning
* risk management

---

## Responsibilities

| Area                | Responsibility        |
| ------------------- | --------------------- |
| Architecture        | approve system design |
| Code Quality        | enforce standards     |
| Technical Decisions | stack validation      |
| Integration         | module coordination   |
| Performance         | oversee optimization  |
| Risk Analysis       | identify blockers     |

---

## Ownership

```text id="lead-ownership"
- system architecture
- repository structure
- module contracts
- technical reviews
```

---

# 2. Native Overlay Engineer

# Highest Complexity Role

---

## Role

Builds:

* overlay rendering system
* transparency
* animations
* capsule behavior

---

## Responsibilities

| Area             | Responsibility        |
| ---------------- | --------------------- |
| Overlay Window   | frameless rendering   |
| Transparency     | blur/glass effects    |
| Window Flags     | Alt+Tab exclusion     |
| Focus Behavior   | non-focus interaction |
| Overlay Movement | drag/reposition       |
| Animation System | transitions           |
| GPU Rendering    | DirectComposition     |

---

## Ownership

```text id="overlay-ownership"
overlay/
native-overlay/
system/windows/
```

---

## Required Skills

| Skill                    | Importance |
| ------------------------ | ---------- |
| Win32 APIs               | Critical   |
| DirectComposition        | High       |
| Rust systems programming | Critical   |
| Graphics rendering       | High       |

---

# 3. Audio Systems Engineer

## Role

Owns:

* microphone capture
* loopback capture
* audio synchronization

---

## Responsibilities

| Area               | Responsibility      |
| ------------------ | ------------------- |
| WASAPI Integration | audio capture       |
| Device Management  | reconnect logic     |
| Buffering          | audio chunking      |
| Synchronization    | timing alignment    |
| Source Labeling    | mic/speaker routing |

---

## Ownership

```text id="audio-ownership"
audio/
```

---

## Required Skills

| Skill            | Importance |
| ---------------- | ---------- |
| WASAPI           | Critical   |
| Real-time audio  | High       |
| Rust concurrency | High       |

---

# 4. Speech Processing Engineer

## Role

Owns:

* Whisper integration
* streaming transcription
* transcript cleanup

---

## Responsibilities

| Area               | Responsibility |
| ------------------ | -------------- |
| Whisper.cpp        | integration    |
| Streaming STT      | live inference |
| Chunk Processing   | segmentation   |
| Transcript Cleanup | normalization  |
| Latency Reduction  | optimization   |

---

## Ownership

```text id="stt-ownership"
stt/
```

---

## Required Skills

| Skill                    | Importance |
| ------------------------ | ---------- |
| Whisper.cpp              | Critical   |
| Audio inference          | High       |
| Performance optimization | High       |

---

# 5. AI Integration Engineer

## Role

Owns:

* AI providers
* streaming responses
* prompt building

---

## Responsibilities

| Area                 | Responsibility      |
| -------------------- | ------------------- |
| AI Providers         | OpenAI/Gemini/etc   |
| Prompt Construction  | context handling    |
| Context Extraction   | transcript chunking |
| Streaming Responses  | token rendering     |
| Hold-to-record Logic | extended context    |

---

## Ownership

```text id="ai-ownership"
ai/
```

---

## Required Skills

| Skill                   | Importance |
| ----------------------- | ---------- |
| AI APIs                 | Critical   |
| Streaming architectures | High       |
| Prompt engineering      | Medium     |

---

# 6. Frontend/UI Engineer

## Role

Owns:

* React frontend
* settings interface
* history panels

---

## Responsibilities

| Area             | Responsibility      |
| ---------------- | ------------------- |
| React UI         | frontend rendering  |
| Settings UX      | configuration       |
| History Viewer   | transcript browsing |
| Export UI        | export management   |
| State Management | frontend store      |

---

## Ownership

```text id="frontend-ownership"
frontend/
```

---

## Required Skills

| Skill       | Importance |
| ----------- | ---------- |
| React       | Critical   |
| TypeScript  | High       |
| TailwindCSS | Medium     |

---

# 7. Persistence Engineer

## Role

Owns:

* SQLite architecture
* transcript persistence
* indexing

---

## Responsibilities

| Area           | Responsibility       |
| -------------- | -------------------- |
| SQLite Schema  | database design      |
| Async Writes   | persistence          |
| Retrieval APIs | history loading      |
| Export Queries | structured retrieval |
| Future Search  | indexing preparation |

---

## Ownership

```text id="db-ownership"
persistence/
```

---

## Required Skills

| Skill                | Importance |
| -------------------- | ---------- |
| SQLite               | Critical   |
| Rust database layers | High       |
| Async systems        | Medium     |

---

# 8. Systems Integration Engineer

## Role

Owns:

* hotkeys
* tray integration
* startup logic
* OS interaction

---

## Responsibilities

| Area                 | Responsibility      |
| -------------------- | ------------------- |
| Tray Integration     | background behavior |
| Global Hotkeys       | Ctrl+Shift+A        |
| Startup Registration | auto-launch         |
| Window Behaviors     | focus/taskbar logic |

---

## Ownership

```text id="system-ownership"
system/
```

---

# 9. Performance Engineer

## Role

Optimizes:

* memory usage
* CPU usage
* rendering smoothness
* latency

---

## Responsibilities

| Area               | Responsibility       |
| ------------------ | -------------------- |
| RAM Optimization   | memory profiling     |
| CPU Optimization   | inference tuning     |
| Async Optimization | runtime efficiency   |
| Overlay FPS        | rendering smoothness |
| STT Latency        | optimization         |

---

## Cross-Team Ownership

Works with:

* overlay
* audio
* STT
* AI

---

# 10. QA / Stability Engineer

## Role

Ensures:

* long-session stability
* crash resistance
* usability

---

## Responsibilities

| Area               | Responsibility        |
| ------------------ | --------------------- |
| Stress Testing     | long meetings         |
| Regression Testing | feature stability     |
| Audio Testing      | reconnect scenarios   |
| Overlay Testing    | rendering reliability |
| Installer Testing  | deployment validation |

---

## Ownership

```text id="qa-ownership"
tests/
```

---

# 11. DevOps / Release Engineer

## Role

Owns:

* builds
* installers
* releases

---

## Responsibilities

| Area               | Responsibility     |
| ------------------ | ------------------ |
| CI/CD              | GitHub Actions     |
| Windows Builds     | MSI/EXE            |
| Release Automation | GitHub releases    |
| Packaging          | installer creation |

---

## Ownership

```text id="devops-ownership"
.github/
scripts/
```

---

# 12. UX/Product Engineer

## Role

Owns:

* interaction quality
* overlay usability
* workflow polish

---

## Responsibilities

| Area                  | Responsibility   |
| --------------------- | ---------------- |
| Overlay Interaction   | UX tuning        |
| Animation Timing      | transitions      |
| Transcript Visibility | readability      |
| AI Interaction UX     | Help Me behavior |

---

# Cross-Team Integration Responsibilities

---

# Overlay + AI

Shared responsibilities:

* streaming response rendering
* token timing
* visual smoothness

---

# Audio + STT

Shared responsibilities:

* audio chunk timing
* latency optimization

---

# Persistence + Export

Shared responsibilities:

* transcript retrieval
* structured exports

---

# Frontend + System

Shared responsibilities:

* settings synchronization
* tray behavior

---

# Recommended Initial Team Allocation

## Phase 1

| Team              | Priority |
| ----------------- | -------- |
| Overlay Engineer  | Critical |
| Audio Engineer    | Critical |
| STT Engineer      | Critical |
| Frontend Engineer | Medium   |

---

# Small-Team Version

If only:

* 1–3 developers

recommended responsibility merge:

| Combined Role     | Areas                  |
| ----------------- | ---------------------- |
| Systems Engineer  | overlay + audio        |
| Backend Engineer  | STT + AI + persistence |
| Frontend Engineer | UI + UX                |

---

# Critical Ownership Rules

## Rule 1

Overlay engineer owns:

```text id="ownership-rule1"
ALL window behavior
```

No scattered Win32 logic.

---

## Rule 2

AI engineer owns:

```text id="ownership-rule2"
ALL provider abstraction
```

No duplicated API code.

---

## Rule 3

Persistence engineer owns:

```text id="ownership-rule3"
ALL SQLite queries
```

No raw DB calls across modules.

---

# Final Engineering Principle

```text id="engineering-principle-final"
Clear ownership
Minimal overlap
Strict module boundaries
```

This responsibility structure is approved for:

* sprint allocation
* team onboarding
* contributor assignment
* open-source collaboration
* engineering management.
