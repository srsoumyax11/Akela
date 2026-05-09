Akela
│
├── EPIC 1 — Project Foundation
│   │
│   ├── Feature 1.1 — Repository Initialization
│   │   │
│   │   ├── Task — Create monorepo
│   │   │   ├── Create root repository
│   │   │   ├── Configure Cargo workspace
│   │   │   ├── Configure Bun workspace
│   │   │   └── Configure pnpm compatibility
│   │   │
│   │   ├── Task — Setup frontend
│   │   │   ├── Initialize React + Vite
│   │   │   ├── Configure TypeScript
│   │   │   └── Setup Zustand
│   │   │
│   │   ├── Task — Setup Tauri
│   │   │   ├── Initialize src-tauri
│   │   │   ├── Configure Tauri v2
│   │   │   ├── Configure transparent window
│   │   │   └── Configure dev commands
│   │   │
│   │   └── Task — Setup CI/CD
│   │       ├── GitHub Actions
│   │       ├── Rust lint workflow
│   │       ├── Frontend lint workflow
│   │       └── Windows build workflow
│   │
│   └── Feature 1.2 — Development Standards
│       │
│       ├── Task — Code formatting
│       │   ├── rustfmt
│       │   ├── clippy
│       │   ├── prettier
│       │   └── eslint
│       │
│       ├── Task — Branch strategy
│       │   ├── main branch
│       │   ├── develop branch
│       │   └── feature branch rules
│       │
│       └── Task — Documentation standards
│           ├── README structure
│           ├── Issue templates
│           └── PR templates
│
├── EPIC 2 — Native Overlay Engine
│   │
│   ├── Feature 2.1 — Transparent Overlay Window
│   │   │
│   │   ├── Task — Create frameless window
│   │   │   ├── Remove decorations
│   │   │   ├── Enable transparency
│   │   │   ├── Configure rounded corners
│   │   │   └── Configure always-on-top
│   │   │
│   │   ├── Task — Window visibility behavior
│   │   │   ├── Hide from taskbar
│   │   │   ├── Hide from Alt+Tab
│   │   │   ├── Non-focus behavior
│   │   │   └── Overlay activation logic
│   │   │
│   │   └── Task — Window movement
│   │       ├── Drag handling
│   │       ├── Position persistence
│   │       └── Multi-monitor positioning
│   │
│   ├── Feature 2.2 — Capsule UI System
│   │   │
│   │   ├── Task — Compact capsule
│   │   │   ├── Capsule layout
│   │   │   ├── Button placement
│   │   │   ├── Transcript ticker
│   │   │   └── Opacity controls
│   │   │
│   │   ├── Task — Expanded overlay
│   │   │   ├── Expansion animations
│   │   │   ├── Transcript panel
│   │   │   ├── AI response panel
│   │   │   └── Settings panel
│   │   │
│   │   └── Task — Overlay animations
│   │       ├── Open animation
│   │       ├── Collapse animation
│   │       ├── Streaming text animation
│   │       └── Blur transitions
│   │
│   └── Feature 2.3 — Tray & Hotkeys
│       │
│       ├── Task — System tray integration
│       │   ├── Tray icon
│       │   ├── Minimize to tray
│       │   └── Exit controls
│       │
│       └── Task — Global hotkeys
│           ├── Ctrl+Shift+A
│           ├── Toggle overlay
│           └── Hotkey conflict handling
│
├── EPIC 3 — Audio Engine
│   │
│   ├── Feature 3.1 — Microphone Capture
│   │   │
│   │   ├── Task — WASAPI microphone input
│   │   │   ├── Detect default mic
│   │   │   ├── Start capture stream
│   │   │   └── Handle reconnects
│   │   │
│   │   └── Task — Audio buffering
│   │       ├── Chunk buffering
│   │       ├── Queue management
│   │       └── Latency optimization
│   │
│   ├── Feature 3.2 — System Audio Capture
│   │   │
│   │   ├── Task — WASAPI loopback
│   │   │   ├── Detect default speaker
│   │   │   ├── Start loopback stream
│   │   │   └── Handle audio resets
│   │   │
│   │   └── Task — Source labeling
│   │       ├── User source tagging
│   │       └── Speaker source tagging
│   │
│   └── Feature 3.3 — Audio Recovery
│       │
│       ├── Task — Device recovery
│       │   ├── Bluetooth reconnects
│       │   ├── USB reconnects
│       │   └── Default device changes
│       │
│       └── Task — Fault tolerance
│           ├── Stream restart
│           ├── Silent recovery
│           └── Error reporting
│
├── EPIC 4 — STT Engine
│   │
│   ├── Feature 4.1 — Whisper Integration
│   │   │
│   │   ├── Task — Integrate whisper.cpp
│   │   │   ├── Load tiny/base models
│   │   │   ├── Configure English mode
│   │   │   └── Configure streaming
│   │   │
│   │   └── Task — Inference pipeline
│   │       ├── Audio chunk processing
│   │       ├── Transcript generation
│   │       └── Low-latency inference
│   │
│   ├── Feature 4.2 — Transcript Processing
│   │   │
│   │   ├── Task — Transcript cleanup
│   │   │   ├── Remove artifacts
│   │   │   ├── Normalize punctuation
│   │   │   └── Merge transcript chunks
│   │   │
│   │   └── Task — Live transcript ticker
│   │       ├── Continuous scrolling
│   │       ├── Overlay rendering
│   │       └── Real-time updates
│   │
│   └── Feature 4.3 — Performance Optimization
│       │
│       ├── Task — CPU optimization
│       │   ├── Quantization tuning
│       │   ├── Thread optimization
│       │   └── Chunk size tuning
│       │
│       └── Task — Memory optimization
│           ├── Buffer cleanup
│           ├── Model memory tuning
│           └── Stream recycling
│
├── EPIC 5 — AI Engine
│   │
│   ├── Feature 5.1 — Provider System
│   │   │
│   │   ├── Task — OpenAI provider
│   │   │   ├── Streaming requests
│   │   │   ├── API key validation
│   │   │   └── Error handling
│   │   │
│   │   ├── Task — Gemini provider
│   │   │   ├── Streaming integration
│   │   │   └── Provider abstraction
│   │   │
│   │   └── Task — NVIDIA provider
│   │       ├── API integration
│   │       └── Unified interface
│   │
│   ├── Feature 5.2 — Context Extraction
│   │   │
│   │   ├── Task — Click mode
│   │   │   ├── Extract last 40–50 words
│   │   │   ├── Build prompt
│   │   │   └── Send AI request
│   │   │
│   │   └── Task — Hold mode
│   │       ├── Continuous accumulation
│   │       ├── Context buffering
│   │       └── Release-to-send
│   │
│   └── Feature 5.3 — Streaming Responses
│       │
│       ├── Task — Token streaming
│       │   ├── SSE handling
│       │   ├── Token buffering
│       │   └── Stream completion
│       │
│       └── Task — Overlay rendering
│           ├── Word-by-word updates
│           ├── Typing effect
│           └── Streaming animations
│
├── EPIC 6 — Persistence Layer
│   │
│   ├── Feature 6.1 — SQLite Integration
│   │   │
│   │   ├── Task — Database initialization
│   │   │   ├── Schema creation
│   │   │   ├── Migration system
│   │   │   └── Connection management
│   │   │
│   │   └── Task — Async persistence
│   │       ├── Transcript writes
│   │       ├── AI response writes
│   │       └── Session writes
│   │
│   ├── Feature 6.2 — Session History
│   │   │
│   │   ├── Task — Session retrieval
│   │   │   ├── Load previous sessions
│   │   │   ├── Load transcripts
│   │   │   └── Load AI logs
│   │   │
│   │   └── Task — Session lifecycle
│   │       ├── Auto session creation
│   │       ├── Session closing
│   │       └── Recovery after crash
│   │
│   └── Feature 6.3 — Export System
│       │
│       ├── Task — TXT export
│       │   ├── Transcript formatting
│       │   └── Metadata inclusion
│       │
│       └── Task — Markdown export
│           ├── Structured formatting
│           ├── AI response sections
│           └── Timestamp formatting
│
├── EPIC 7 — Settings & Configuration
│   │
│   ├── Feature 7.1 — Provider Settings
│   │   │
│   │   ├── Task — API key management
│   │   │   ├── OpenAI keys
│   │   │   ├── Gemini keys
│   │   │   └── NVIDIA keys
│   │   │
│   │   └── Task — Provider switching
│   │       ├── Save preferences
│   │       └── Active provider handling
│   │
│   ├── Feature 7.2 — Overlay Settings
│   │   │
│   │   ├── Task — Opacity controls
│   │   │   ├── Transparency slider
│   │   │   └── Real-time updates
│   │   │
│   │   └── Task — Startup settings
│   │       ├── Open at startup
│   │       ├── Open settings toggle
│   │       └── Tray behavior
│   │
│   └── Feature 7.3 — System Prompts
│       │
│       ├── Task — Prompt editor
│       │   ├── Save prompt
│       │   └── Load prompt
│       │
│       └── Task — Prompt injection
│           ├── Merge with context
│           └── AI request integration
│
└── EPIC 8 — Stabilization & Release
    │
    ├── Feature 8.1 — Performance Optimization
    │   │
    │   ├── Task — Overlay optimization
    │   │   ├── GPU tuning
    │   │   ├── Animation optimization
    │   │   └── FPS stabilization
    │   │
    │   ├── Task — Audio optimization
    │   │   ├── Latency reduction
    │   │   ├── Buffer tuning
    │   │   └── CPU tuning
    │   │
    │   └── Task — STT optimization
    │       ├── Memory tuning
    │       ├── Quantization tuning
    │       └── Inference optimization
    │
    ├── Feature 8.2 — QA Testing
    │   │
    │   ├── Task — Overlay stress testing
    │   │   ├── Long session tests
    │   │   ├── Multi-monitor tests
    │   │   └── Transparency tests
    │   │
    │   ├── Task — Audio stability testing
    │   │   ├── Device reconnect tests
    │   │   ├── Bluetooth tests
    │   │   └── Failure recovery tests
    │   │
    │   └── Task — AI integration testing
    │       ├── Streaming interruption tests
    │       ├── API failure tests
    │       └── Timeout handling
    │
    └── Feature 8.3 — Packaging & Release
        │
        ├── Task — Windows packaging
        │   ├── MSI installer
        │   ├── EXE packaging
        │   └── Icon/resources
        │
        ├── Task — GitHub releases
        │   ├── Release workflow
        │   ├── Changelog generation
        │   └── Release artifacts
        │
        └── Task — MVP release
            ├── Version tagging
            ├── Documentation publish
            └── Public release