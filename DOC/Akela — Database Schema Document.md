# Akela — Database Schema Document

## Database Overview

Akela uses:

```text id="db-engine"
SQLite
```

Reasoning:

* lightweight
* embedded
* zero backend
* ideal for desktop applications
* fast local reads/writes
* easy export/import

---

# Database Goals

The database is designed for:

| Goal                   | Description           |
| ---------------------- | --------------------- |
| Local-first storage    | no remote dependency  |
| Fast transcript writes | real-time persistence |
| Session history        | meeting archive       |
| AI request logging     | contextual history    |
| Export support         | structured retrieval  |
| Future search support  | transcript indexing   |

---

# Database File Location

## Windows

Recommended location:

```text id="db-path"
%APPDATA%/Akela/akela.db
```

Additional folders:

```text id="app-storage"
%APPDATA%/Akela/
├── akela.db
├── exports/
├── logs/
└── config/
```

---

# Database Structure Overview

```text id="db-overview"
sessions
transcripts
ai_requests
settings
exports
```

---

# 1. sessions Table

## Purpose

Stores:

* meeting metadata
* session configuration
* prompts
* timestamps

---

## Schema

```sql id="sessions-schema"
CREATE TABLE sessions (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    system_prompt TEXT,
    language TEXT DEFAULT 'en',
    created_at DATETIME NOT NULL,
    ended_at DATETIME,
    is_active BOOLEAN DEFAULT 1
);
```

---

## Field Breakdown

| Field         | Type       | Description                |
| ------------- | ---------- | -------------------------- |
| id            | TEXT(UUID) | unique session ID          |
| title         | TEXT       | user-defined meeting title |
| description   | TEXT       | optional notes             |
| system_prompt | TEXT       | AI behavior instructions   |
| language      | TEXT       | transcription language     |
| created_at    | DATETIME   | session start              |
| ended_at      | DATETIME   | session end                |
| is_active     | BOOLEAN    | current active session     |

---

# 2. transcripts Table

## Purpose

Stores:

* all live transcription data
* speaker classification
* timestamps

---

## Schema

```sql id="transcripts-schema"
CREATE TABLE transcripts (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    source TEXT NOT NULL,
    text TEXT NOT NULL,
    created_at DATETIME NOT NULL,

    FOREIGN KEY(session_id)
    REFERENCES sessions(id)
    ON DELETE CASCADE
);
```

---

## Field Breakdown

| Field      | Type       | Description          |
| ---------- | ---------- | -------------------- |
| id         | TEXT(UUID) | transcript chunk ID  |
| session_id | TEXT       | linked meeting       |
| source     | TEXT       | `user` or `speaker`  |
| text       | TEXT       | transcript content   |
| created_at | DATETIME   | transcript timestamp |

---

## Source Values

Allowed:

```text id="transcript-sources"
user
speaker
```

Meaning:

* `user` → microphone
* `speaker` → system audio

---

# 3. ai_requests Table

## Purpose

Stores:

* AI prompts
* generated responses
* context windows

---

## Schema

```sql id="ai-schema"
CREATE TABLE ai_requests (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,

    trigger_type TEXT NOT NULL,
    context_text TEXT NOT NULL,

    provider TEXT NOT NULL,
    model TEXT,

    response TEXT NOT NULL,

    created_at DATETIME NOT NULL,

    FOREIGN KEY(session_id)
    REFERENCES sessions(id)
    ON DELETE CASCADE
);
```

---

## Field Breakdown

| Field        | Type       | Description           |
| ------------ | ---------- | --------------------- |
| id           | TEXT(UUID) | request ID            |
| session_id   | TEXT       | linked session        |
| trigger_type | TEXT       | click/hold            |
| context_text | TEXT       | transcript sent to AI |
| provider     | TEXT       | OpenAI/Gemini/etc     |
| model        | TEXT       | model identifier      |
| response     | TEXT       | generated AI response |
| created_at   | DATETIME   | request timestamp     |

---

## Trigger Types

```text id="trigger-types"
click
hold
```

---

# 4. settings Table

## Purpose

Stores:

* local application preferences
* UI configuration
* startup behavior

---

## Schema

```sql id="settings-schema"
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
```

---

## Example Values

| key                   | value  |
| --------------------- | ------ |
| overlay_opacity       | 0.8    |
| open_settings_startup | true   |
| selected_provider     | openai |
| startup_enabled       | true   |

---

# 5. exports Table

## Purpose

Tracks:

* export history
* exported sessions
* formats

---

## Schema

```sql id="exports-schema"
CREATE TABLE exports (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,

    export_type TEXT NOT NULL,
    export_path TEXT NOT NULL,

    created_at DATETIME NOT NULL,

    FOREIGN KEY(session_id)
    REFERENCES sessions(id)
    ON DELETE CASCADE
);
```

---

# Export Types

```text id="export-types"
txt
markdown
pdf
```

---

# Recommended Indexes

## Transcript Index

```sql id="transcript-index"
CREATE INDEX idx_transcripts_session
ON transcripts(session_id);
```

---

## Transcript Time Index

```sql id="transcript-time-index"
CREATE INDEX idx_transcripts_created
ON transcripts(created_at);
```

---

## AI Request Index

```sql id="ai-index"
CREATE INDEX idx_ai_session
ON ai_requests(session_id);
```

---

# Future Search Optimization

## Planned Future Feature

Later versions may add:

```sql id="fts-schema"
CREATE VIRTUAL TABLE transcript_search
USING fts5(text);
```

For:

* fast transcript search
* keyword retrieval
* semantic filtering

NOT required for V1.

---

# Database Write Strategy

## Real-Time Persistence

Transcripts are:

* incrementally written
* chunked
* async persisted

Recommended interval:

```text id="write-interval"
every few transcript chunks
```

NOT:

* every word
* full meeting buffer

This improves:

* SSD health
* write performance
* CPU usage

---

# Recommended ORM/Database Layer

## Recommended Choice

```text id="db-layer"
sqlx
```

Reason:

* async support
* compile-time query checking
* SQLite support
* Rust-native

Alternative:

```text id="db-alt"
rusqlite
```

for simpler synchronous usage.

---

# Session Lifecycle Example

```text id="session-flow"
Create Session
    ↓
Store Transcript Chunks
    ↓
Store AI Requests
    ↓
Export Session
    ↓
Close Session
```

---

# Data Retention Policy

## Current Policy

```text id="retention-policy"
Persistent until user deletes
```

No automatic cleanup in V1.

---

# Database Safety Strategy

## V1 Decisions

| Area        | Decision |
| ----------- | -------- |
| Encryption  | No       |
| Compression | No       |
| Cloud Sync  | No       |
| Multi-user  | No       |

Keeps architecture simple.

---

# Backup Strategy

## Recommended Future Enhancement

Automatic:

* periodic DB backup
* export snapshots

NOT included in V1.

---

# Example Session Record

```json id="session-example"
{
  "id": "uuid",
  "title": "Weekly Research Meeting",
  "description": "ML model discussion",
  "system_prompt": "Respond professionally",
  "language": "en",
  "created_at": "2026-05-09T12:00:00Z"
}
```

---

# Example Transcript Record

```json id="transcript-example"
{
  "id": "uuid",
  "session_id": "uuid",
  "source": "speaker",
  "text": "What is the estimated cost of the deployment?",
  "created_at": "2026-05-09T12:04:21Z"
}
```

---

# Example AI Request Record

```json id="ai-example"
{
  "id": "uuid",
  "session_id": "uuid",
  "trigger_type": "click",
  "provider": "openai",
  "context_text": "What is the estimated cost of the deployment?",
  "response": "The deployment cost depends on...",
  "created_at": "2026-05-09T12:04:26Z"
}
```

---

# Final Database Design Principles

```text id="db-principles"
Fast
Simple
Local-first
Async-friendly
Future-searchable
Export-oriented
```

This schema is approved for:

* MVP implementation
* migration generation
* persistence module development
* transcript storage integration.
