# Native Overlay System

Akela's "Capsule" UI is more than just a transparent window; it's a deeply integrated Windows component designed for maximum stealth and zero friction.

---

## The Stealth Principle
Most "overlay" apps suffer from focus stealing or being visible in screen captures. Akela solves this using native Win32 APIs.

### 1. Invisibility to Screen Capture
We use `SetWindowDisplayAffinity` with `WDA_EXCLUDEFROMCAPTURE`. This ensures that even if you are recording your screen or sharing your desktop in a meeting, the Akela overlay is **completely invisible** to everyone but you.

### 2. Zero-Focus Interaction
To ensure the app never interrupts your work:
- **Exclusion**: The window is excluded from the Taskbar and the Alt+Tab menu using `GWL_EXSTYLE` flags (`WS_EX_TOOLWINDOW` and `WS_EX_NOACTIVATE`).
- **Click-Through (Optional)**: In certain modes, we can enable `WS_EX_TRANSPARENT` to allow clicks to pass through the overlay to the window behind it.

---

## Window Lifecycle

### Initialization
Upon startup, the Tauri window is configured as:
- **Decorations**: Disabled (no title bar or borders).
- **Transparency**: Enabled at the OS level.
- **Always-on-Top**: Set to ensure it floats above all other apps.

### Movement & Persistence
Even though the window is frameless, it supports dragging.
- **Native Dragging**: We use Tauri's `start_dragging` to move the window efficiently.
- **Position Persistence**: The window's X/Y coordinates are saved to a local config file and restored on the next launch, even in multi-monitor setups.

---

## Technical Details

| Feature | Win32 Flag / API | Purpose |
| --- | --- | --- |
| **Hide from Taskbar** | `WS_EX_TOOLWINDOW` | Keep the workspace clean. |
| **No Focus** | `WS_EX_NOACTIVATE` | Don't steal keyboard focus. |
| **Stealth** | `WDA_EXCLUDEFROMCAPTURE` | Hide from screenshots/recordings. |
| **Rounded Corners** | `DWMWA_WINDOW_CORNER_PREFERENCE` | Modern Windows 11 aesthetics. |
| **Layered Window** | `WS_EX_LAYERED` | Required for alpha-channel transparency. |
