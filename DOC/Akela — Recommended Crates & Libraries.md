# Akela — Recommended Crates & Libraries

## Selection Philosophy

Akela requires libraries that are:

```text id="selection-philosophy"
Lightweight
Native-friendly
Stable
Async-compatible
Windows-capable
Performance-oriented
```

Avoid:

* bloated abstractions
* Electron-style heavy runtimes
* unnecessary dependency chains

---

# 1. Core Runtime & Architecture

## Tokio

### Purpose

Async runtime.

---

### Why

Needed for:

* streaming STT
* AI streaming
* overlay events
* async DB writes
* concurrent audio processing

---

## Crate

```toml id="tokio-crate"
tokio = { version = "1", features = ["full"] }
```

---

# 2. Windows Native APIs

## windows

### Purpose

Modern Win32 bindings.

---

### Why

Needed for:

* overlay windows
* transparency
* focus control
* Alt+Tab exclusion
* tray integration
* WASAPI

---

## Crate

```toml id="windows-crate"
windows = { version = "0.56", features = [...] }
```

---

## Important APIs Used

| Area         | APIs              |
| ------------ | ----------------- |
| Overlay      | Win32 Window APIs |
| Transparency | DWM               |
| Audio        | WASAPI            |
| Focus        | Window styles     |
| Tray         | Shell APIs        |

---

# 3. Tauri

## Purpose

Application shell.

Handles:

* packaging
* updater
* frontend bridge
* tray support

---

## Crate

```toml id="tauri-crate"
tauri = "2"
```

---

# 4. Frontend Stack

## React

### Purpose

Settings/history UI.

---

## Packages

```json id="react-packages"
react
react-dom
```

---


---

## Zustand

### Purpose

Lightweight state management.

---

## Why

Simpler than Redux.

Good for:

* overlay state
* transcript state
* settings state

---

## Package

```json id="zustand-package"
zustand
```

---

# 5. Audio Processing

# Recommended Primary Choice

## cpal

### Purpose

Cross-platform audio abstraction.

---

### Why

Good integration layer for:

* microphone capture
* device enumeration

---

## Crate

```toml id="cpal-crate"
cpal = "0.15"
```

---

# Windows-Specific Layer

## windows WASAPI APIs

Needed for:

```text id="wasapi-usage"
loopback system audio capture
```

Because:

* cpal alone is insufficient for advanced loopback capture.

---

# 6. STT / Whisper Integration

## whisper.cpp

### Purpose

Fast local speech-to-text.

---

## Integration Strategy

Use:

```text id="whisper-strategy"
FFI bindings
```

NOT:

* spawning subprocesses

---

## Recommended Crates

### whisper-rs

```toml id="whisper-rs"
whisper-rs = "0.11"
```

---

## Why

Provides:

* Rust bindings
* streaming-friendly integration
* low overhead

---

# 7. Database Layer

# Recommended Choice

## sqlx

### Purpose

Async SQLite access.

---

## Why

Provides:

* compile-time query validation
* async support
* good SQLite integration

---

## Crate

```toml id="sqlx-crate"
sqlx = { version = "0.7", features = ["sqlite", "runtime-tokio-rustls"] }
```

---

# Migration Support

## sqlx migrate

For:

* schema versioning
* DB upgrades

---

# 8. Serialization

## serde

### Purpose

JSON/config serialization.

---

## Crates

```toml id="serde-crates"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

---

# 9. AI Networking

## reqwest

### Purpose

HTTP requests + streaming.

---

## Why

Needed for:

* OpenAI APIs
* Gemini APIs
* streaming token responses

---

## Crate

```toml id="reqwest-crate"
reqwest = { version = "0.12", features = ["json", "stream"] }
```

---

# Streaming Support

## futures-util

### Purpose

Streaming token handling.

---

## Crate

```toml id="futures-crate"
futures-util = "0.3"
```

---

# 10. Event System

# Recommended Approach

## Tokio Channels

### Purpose

Internal module communication.

---

## APIs

```text id="tokio-events"
tokio::sync::mpsc
tokio::sync::broadcast
```

---

# Why

Needed for:

* transcript events
* AI token streams
* overlay updates

---

# 11. Global Hotkeys

## global-hotkey

### Purpose

Global shortcut registration.

---

## Crate

```toml id="hotkey-crate"
global-hotkey = "0.5"
```

---

# Used For

```text id="hotkey-usage"
Ctrl + Shift + A
```

---

# 12. System Tray

## Tauri Tray APIs

### Purpose

Tray minimization.

---

## Why

Already integrated with:

* Windows tray behavior
* Tauri runtime

---

# 13. Logging

## tracing

### Purpose

Structured async logging.

---

## Crates

```toml id="tracing-crates"
tracing = "0.1"
tracing-subscriber = "0.3"
```

---

# Why

Needed for:

* audio debugging
* overlay debugging
* AI streaming diagnostics

---

# 14. Configuration Management

## confy

### Purpose

Simple local settings persistence.

---

## Crate

```toml id="confy-crate"
confy = "0.6"
```

---

# Stores

| Setting          | Example |
| ---------------- | ------- |
| API provider     | OpenAI  |
| Overlay opacity  | 0.8     |
| Startup behavior | true    |

---

# 15. UUID Generation

## uuid

### Purpose

Session IDs.

---

## Crate

```toml id="uuid-crate"
uuid = { version = "1", features = ["v4"] }
```

---

# 16. Time Handling

## chrono

### Purpose

Timestamps.

---

## Crate

```toml id="chrono-crate"
chrono = "0.4"
```

---

# 17. File Dialogs

## rfd

### Purpose

Export file selection.

---

## Crate

```toml id="rfd-crate"
rfd = "0.14"
```

---

# 18. Markdown Export

## pulldown-cmark

### Purpose

Markdown formatting/rendering.

---

## Crate

```toml id="markdown-crate"
pulldown-cmark = "0.10"
```

---

# 19. PDF Export (Future)

## printpdf

### Purpose

PDF generation.

---

## Crate

```toml id="pdf-crate"
printpdf = "0.7"
```

---

# 20. Animation Utilities

## tauri-plugin-positioner

(Optional)

### Purpose

Overlay placement utilities.

---

# 21. Clipboard Support

## arboard

### Purpose

Copy AI responses.

---

## Crate

```toml id="clipboard-crate"
arboard = "3"
```

---

# 22. Recommended Cargo Workspace

## Root Cargo.toml

```toml id="workspace-example"
[workspace]
members = [
    "src-tauri",
    "native-overlay"
]
```

---

# Recommended Dependency Philosophy

## KEEP

```text id="keep-philosophy"
Small focused crates
```

---

## AVOID

```text id="avoid-philosophy"
Massive UI/game-engine ecosystems
```

Examples to avoid:

* full game engines
* Electron-native bridges
* giant GUI frameworks

---

# Crates We Intentionally Avoid

| Crate               | Reason                  |
| ------------------- | ----------------------- |
| Electron            | memory heavy            |
| GTK                 | unnecessary for overlay |
| Qt                  | huge dependency size    |
| Bevy                | overkill                |
| Tauri-only overlays | insufficient control    |

---

# Final Recommended Core Stack

| Area          | Primary Library |
| ------------- | --------------- |
| Runtime       | Tokio           |
| Windows APIs  | windows         |
| Shell         | Tauri           |
| Frontend      | React           |
| Audio         | cpal + WASAPI   |
| STT           | whisper-rs      |
| DB            | sqlx            |
| AI Networking | reqwest         |
| Logging       | tracing         |
| Hotkeys       | global-hotkey   |

---

# Final Dependency Philosophy

```text id="dependency-principle"
Native-first
Minimal abstraction
Performance-sensitive
Overlay-oriented
```

This crate/library selection is approved for:

* repository initialization
* dependency locking
* MVP implementation
* prototype development
* long-term maintainability.
