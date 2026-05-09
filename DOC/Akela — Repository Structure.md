# Akela — Repository Structure

## Repository Strategy

```text id="repo-type"
Monorepo Architecture
```

Reason:

* single deployment
* shared Rust modules
* easier dependency management
* simpler open-source contribution flow
* unified build pipeline

---

# Top-Level Repository Layout

```text id="repo-root"
akela/
│
├── .github/
├── docs/
├── frontend/
├── src-tauri/
├── native-overlay/
├── scripts/
├── assets/
├── models/
├── tests/
├── tools/
│
├── .gitignore
├── LICENSE
├── README.md
├── package.json
├── pnpm-workspace.yaml
└── Cargo.toml
```

---

# 1. `.github/`

GitHub workflows and contribution automation.

```text id="github-structure"
.github/
│
├── workflows/
│   ├── build.yml
│   ├── release.yml
│   └── lint.yml
│
├── ISSUE_TEMPLATE/
└── pull_request_template.md
```

---

# 2. `docs/`

All architecture and engineering documentation.

```text id="docs-structure"
docs/
│
├── architecture/
│   ├── system-architecture.md
│   ├── overlay-rendering.md
│   ├── audio-pipeline.md
│   ├── ai-engine.md
│   └── database-design.md
│
├── api/
├── ui-ux/
├── roadmap/
└── decisions/
```

---

# 3. `frontend/`

React + TypeScript frontend.

Purpose:

* settings UI
* transcript history
* exports
* onboarding
* configuration panels

---

## Frontend Structure

```text id="frontend-structure"
frontend/
│
├── public/
│
├── src/
│   ├── components/
│   ├── pages/
│   ├── layouts/
│   ├── hooks/
│   ├── store/
│   ├── services/
│   ├── styles/
│   ├── types/
│   ├── utils/
│   └── main.tsx
│
├── package.json
├── tsconfig.json
├── vite.config.ts
└── tailwind.config.js
```

---

# Frontend Modules

## `components/`

Reusable UI.

```text id="frontend-components"
components/
├── overlay/
├── transcript/
├── settings/
├── session/
├── export/
└── common/
```

---

## `pages/`

Logical screens/states.

```text id="frontend-pages"
pages/
├── onboarding/
├── settings/
├── history/
└── exports/
```

---

## `store/`

Global state management.

Recommended:

```text id="frontend-store"
zustand
```

Stores:

* overlay state
* transcript state
* AI response state
* settings state

---

# 4. `src-tauri/`

Main Rust backend.

This is the core application engine.

---

## Structure

```text id="src-tauri-structure"
src-tauri/
│
├── src/
│   ├── audio/
│   ├── stt/
│   ├── ai/
│   ├── overlay/
│   ├── persistence/
│   ├── session/
│   ├── export/
│   ├── system/
│   ├── config/
│   ├── events/
│   ├── utils/
│   └── main.rs
│
├── Cargo.toml
├── tauri.conf.json
└── build.rs
```

---

# Backend Module Breakdown

---

## `audio/`

Audio capture and routing.

```text id="audio-structure"
audio/
├── mic_capture.rs
├── system_capture.rs
├── device_manager.rs
├── audio_buffer.rs
└── mod.rs
```

Responsibilities:

* WASAPI integration
* microphone capture
* loopback capture
* device recovery

---

## `stt/`

Speech-to-text engine.

```text id="stt-structure"
stt/
├── whisper.rs
├── streaming.rs
├── chunking.rs
├── transcript_cleaner.rs
└── mod.rs
```

Responsibilities:

* Whisper.cpp integration
* transcript streaming
* chunk buffering
* cleanup/punctuation

---

## `ai/`

AI provider abstraction.

```text id="ai-structure"
ai/
├── providers/
│   ├── openai.rs
│   ├── gemini.rs
│   ├── nvidia.rs
│   └── mod.rs
│
├── prompt_builder.rs
├── context_extractor.rs
├── streaming.rs
└── mod.rs
```

Responsibilities:

* provider abstraction
* prompt construction
* streaming responses

---

## `overlay/`

Native overlay engine.

```text id="overlay-structure"
overlay/
├── renderer.rs
├── capsule.rs
├── transparency.rs
├── blur.rs
├── animations.rs
├── hotkeys.rs
├── tray.rs
└── mod.rs
```

Responsibilities:

* overlay rendering
* transparency
* blur effects
* capsule animation
* tray integration

---

## `persistence/`

SQLite storage layer.

```text id="persistence-structure"
persistence/
├── db.rs
├── sessions.rs
├── transcripts.rs
├── ai_logs.rs
├── migrations/
└── mod.rs
```

Responsibilities:

* SQLite management
* transcript persistence
* session indexing

---

## `session/`

Meeting/session lifecycle.

```text id="session-structure"
session/
├── manager.rs
├── state.rs
├── metadata.rs
└── mod.rs
```

Responsibilities:

* active session management
* session metadata
* session lifecycle

---

## `export/`

Export functionality.

```text id="export-structure"
export/
├── markdown.rs
├── txt.rs
├── pdf.rs
└── mod.rs
```

Responsibilities:

* transcript exports
* AI response exports

---

## `system/`

Windows-specific integration.

```text id="system-structure"
system/
├── windows/
│   ├── focus.rs
│   ├── alt_tab.rs
│   ├── startup.rs
│   ├── window_flags.rs
│   └── mod.rs
│
└── mod.rs
```

Responsibilities:

* Win32 integration
* non-focus windows
* Alt+Tab exclusion
* startup registration

---

## `events/`

Internal event system.

```text id="events-structure"
events/
├── bus.rs
├── transcript_events.rs
├── overlay_events.rs
├── ai_events.rs
└── mod.rs
```

Responsibilities:

* async communication
* internal pub/sub

---

# 5. `native-overlay/`

Dedicated experimental overlay renderer.

Purpose:

* isolated rendering experiments
* DirectComposition testing
* transparency testing

---

## Structure

```text id="native-overlay-structure"
native-overlay/
│
├── src/
│   ├── renderer/
│   ├── composition/
│   ├── transparency/
│   ├── shaders/
│   └── main.rs
│
└── Cargo.toml
```

---

# 6. `scripts/`

Development scripts.

```text id="scripts-structure"
scripts/
├── setup.ps1
├── build.ps1
├── release.ps1
└── dev.ps1
```

---

# 7. `assets/`

Application assets.

```text id="assets-structure"
assets/
├── icons/
├── fonts/
├── images/
├── sounds/
└── themes/
```

---

# 8. `models/`

Local AI/STT assets.

```text id="models-structure"
models/
├── whisper/
└── future/
```

---

# 9. `tests/`

Testing infrastructure.

```text id="tests-structure"
tests/
├── integration/
├── overlay/
├── audio/
├── ai/
└── persistence/
```

---

# Build System

## Frontend

```text id="frontend-build"
Vite
```

---

## Rust

```text id="rust-build"
Cargo
```

---

## Package Manager

Recommended:

```text id="package-manager"
pnpm
```

---

# Branch Strategy

## Recommended

```text id="branch-strategy"
main
develop
feature/*
```

---

# Initial Development Priority

Recommended order:

```text id="dev-order"
1. Native overlay prototype
2. WASAPI audio capture
3. Whisper streaming
4. SQLite persistence
5. AI streaming
6. React/Tauri shell
7. Export system
```

---

# Repository Ownership Model

## Open Source Workflow

Recommended:

* Issues enabled
* Discussions enabled
* PR-based contribution flow
* Conventional commits

---

# Final Repository Philosophy

```text id="repo-philosophy"
Single repository
Single executable
Modular internal architecture
Native-first overlay system
```

This repository structure is approved for:

* MVP development
* open-source collaboration
* long-term maintainability
* future Linux expansion.
