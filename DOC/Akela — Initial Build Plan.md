# Akela — Initial Build Plan

## Build Strategy Overview

Akela uses:

```text id="build-stack"
Rust + Tauri + React + Bun
```

Architecture:

* Rust handles backend + overlay
* Tauri handles shell
* React handles frontend
* Bun handles frontend package/runtime tooling

---

# Build Objectives

The initial build plan must achieve:

| Goal                     | Description              |
| ------------------------ | ------------------------ |
| Fast local development   | low startup latency      |
| Native Windows builds    | MSI/EXE support          |
| Monorepo simplicity      | unified tooling          |
| Open-source friendliness | easy contributor setup   |
| Lightweight runtime      | avoid Node-heavy tooling |

---

# Final Toolchain

| Layer               | Tool  |
| ------------------- | ----- |
| Frontend Runtime    | Bun   |
| Frontend Build Tool | Vite  |
| Frontend Framework  | React |
| Desktop Shell       | Tauri |
| Backend Language    | Rust  |
| Package Manager     | Bun   |
| Rust Build System   | Cargo |

---

# Required Installations

## 1. Rust

Install:

* Rust stable
* Cargo

Recommended:

[Rust Installation](https://www.rust-lang.org/tools/install?utm_source=chatgpt.com)

---

## 2. Bun

Install Bun runtime/package manager.

[Bun Installation](https://bun.sh/docs/installation?utm_source=chatgpt.com)

---

## 3. Visual Studio Build Tools

Required for:

* Win32 APIs
* native linking
* Tauri Windows builds

Install:

* Desktop development with C++

[Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/?utm_source=chatgpt.com)

---

# Repository Initialization

## Step 1 — Create Repository

```bash id="repo-init"
mkdir akela
cd akela
```

---

# Step 2 — Initialize Bun Frontend

```bash id="bun-init"
bun create vite frontend --template react-ts
```

---

# Step 3 — Install Frontend Dependencies

```bash id="bun-install"
cd frontend

bun install
bun add tailwindcss postcss autoprefixer zustand
```

---

# Step 4 — Initialize Tailwind

```bash id="tailwind-init"
bunx tailwindcss init -p
```

---

# Step 5 — Return To Root

```bash id="root-return"
cd ..
```

---

# Step 6 — Create Tauri Application

## Initialize Tauri

```bash id="tauri-init"
bunx tauri init
```

Recommended answers:

| Question      | Value                                          |
| ------------- | ---------------------------------------------- |
| App name      | Akela                                          |
| Window title  | Akela                                          |
| Frontend path | frontend                                       |
| Dev server    | [http://localhost:5173](http://localhost:5173) |
| Build command | bun run build                                  |

---

# Final Repository Structure

```text id="final-structure"
akela/
│
├── frontend/
├── src-tauri/
├── native-overlay/
├── docs/
├── scripts/
└── Cargo.toml
```

---

# Frontend Build Configuration

## package.json

Inside:

```text id="package-location"
frontend/package.json
```

Scripts:

```json id="package-scripts"
{
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  }
}
```

---

# Tauri Configuration

## tauri.conf.json

Inside:

```text id="tauri-config-location"
src-tauri/tauri.conf.json
```

---

## Important Initial Settings

### Disable Decorations

```json id="tauri-window"
{
  "windows": [
    {
      "decorations": false,
      "transparent": true,
      "alwaysOnTop": true
    }
  ]
}
```

This is temporary.
Final overlay logic moves to native renderer later.

---

# Root Cargo Workspace

## Root Cargo.toml

```toml id="workspace-config"
[workspace]
members = [
    "src-tauri",
    "native-overlay"
]
```

---

# Native Overlay Crate

## Create Overlay Crate

```bash id="overlay-crate"
cargo new native-overlay
```

---

# Overlay Dependencies

## native-overlay/Cargo.toml

```toml id="overlay-deps"
[dependencies]
windows = "0.56"
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
```

---

# Main Rust Dependencies

## src-tauri/Cargo.toml

Recommended initial dependencies:

```toml id="main-rust-deps"
[dependencies]
tauri = "2"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
uuid = { version = "1", features = ["v4"] }
chrono = "0.4"
sqlx = { version = "0.7", features = ["sqlite", "runtime-tokio-rustls"] }
reqwest = { version = "0.12", features = ["json", "stream"] }
```

---

# Initial Frontend Dependencies

## Required Bun Packages

```bash id="frontend-deps"
bun add react react-dom zustand
bun add -d typescript vite @types/react @types/react-dom
```

---

# Development Workflow

## Start Frontend

```bash id="frontend-dev"
cd frontend
bun run dev
```

---

## Start Tauri Application

```bash id="tauri-dev"
bunx tauri dev
```

---

# Initial MVP Build Order

## Step 1

```text id="build-order-1"
Transparent overlay window
```

---

## Step 2

```text id="build-order-2"
Movable capsule UI
```

---

## Step 3

```text id="build-order-3"
Native Win32 overlay integration
```

---

## Step 4

```text id="build-order-4"
Audio capture
```

---

## Step 5

```text id="build-order-5"
Whisper integration
```

---

# Recommended Initial Git Setup

## .gitignore

```gitignore id="gitignore"
target/
node_modules/
dist/
src-tauri/target/
.env
*.db
```

---

# Environment Variables

## Recommended `.env`

```env id="env-example"
OPENAI_API_KEY=
GEMINI_API_KEY=
NVIDIA_API_KEY=
```

---

# Development Modes

| Mode                | Purpose               |
| ------------------- | --------------------- |
| Frontend-only       | rapid UI iteration    |
| Tauri dev           | integrated app        |
| Native overlay test | rendering experiments |

---

# Initial Prototype Goals

The FIRST working build should ONLY prove:

```text id="prototype-goals"
1. Transparent overlay
2. Movable capsule
3. Always-on-top
4. No taskbar presence
5. Expand/collapse
```

NOT:

* STT
* AI
* persistence

Those come later.

---

# Recommended Initial Branches

```text id="initial-branches"
main
develop
feature/overlay-prototype
```

---

# Build Pipeline (Future)

## Planned CI

```text id="future-ci"
GitHub Actions
```

Will later build:

* Windows EXE
* MSI installer
* release artifacts

---

# Initial Build Success Criteria

The first successful build is considered complete when:

```text id="initial-success"
Akela launches
    ↓
Floating transparent capsule appears
    ↓
Overlay is movable
    ↓
Overlay stays on top
    ↓
Overlay minimizes to tray
```

with:

* stable rendering
* no crashes
* smooth interaction

---

# Final Initial Build Principle

```text id="build-principle"
Validate overlay architecture first.
Everything else depends on it.
```

This build plan is approved for:

* repository initialization
* environment setup
* prototype implementation
* first engineering sprint.
