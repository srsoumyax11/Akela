use tauri::{Window, Manager, AppHandle};
use std::sync::Mutex;
use crate::window;
use crate::audio::engine::AudioEngine;

#[tauri::command]
pub fn hide_window(window: Window) {
    if let Some(webview) = window.get_webview_window("main") {
        window::hide_window_native(&webview);
    }
}

#[tauri::command]
pub fn show_window_no_focus(window: Window) {
    if let Some(webview) = window.get_webview_window("main") {
        window::show_window_no_focus(&webview);
    }
}

#[tauri::command]
pub fn move_window_no_focus(window: Window, x: i32, y: i32) {
    if let Some(webview) = window.get_webview_window("main") {
        window::move_window_native(&webview, x, y);
    }
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Managed state for the audio engine
pub struct AudioEngineState(pub Mutex<Option<AudioEngine>>);

/// Start the dual-pipeline audio engine.
///
/// The frontend calls this when the user starts a meeting session.
/// `model_path` is the path to the Whisper GGML model file.
/// Supports absolute paths, and relative paths (resolved from exe dir, then CWD).
#[tauri::command]
pub async fn start_audio_engine(
    app: AppHandle,
    state: tauri::State<'_, AudioEngineState>,
    model_path: String,
) -> Result<String, String> {
    let mut engine_guard = state.0.lock().map_err(|e| format!("Lock error: {e}"))?;

    if engine_guard.is_some() {
        return Err("Audio engine is already running".into());
    }

    // Resolve model path — try multiple locations
    let raw = std::path::PathBuf::from(&model_path);
    let resolved = if raw.is_absolute() && raw.exists() {
        raw
    } else {
        // Try relative to the executable directory (prod)
        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.to_path_buf()));

        let candidates = [
            // Relative to exe dir
            exe_dir.as_ref().map(|d| d.join(&model_path)),
            // Relative to CWD (dev mode — CWD is often src-tauri)
            Some(std::env::current_dir().unwrap_or_default().join(&model_path)),
            // Try going up from src-tauri to project root
            exe_dir.as_ref().map(|d| d.join("..").join(&model_path)),
            // Direct from project root in dev (CWD/..)
            Some(std::env::current_dir().unwrap_or_default().join("..").join("models").join("ggml-base.en.bin")),
            // Hardcoded fallback for the known project structure
            Some(std::path::PathBuf::from(r"d:\AppDev\Akela\models\ggml-base.en.bin")),
        ];

        let mut found = None;
        for candidate in candidates.into_iter().flatten() {
            let canonical = candidate.canonicalize().unwrap_or(candidate.clone());
            tracing::debug!("Checking model path: {}", canonical.display());
            if canonical.exists() {
                tracing::info!("Found model at: {}", canonical.display());
                found = Some(canonical);
                break;
            }
        }

        found.ok_or_else(|| format!("Model file not found. Tried: {model_path}"))?
    };

    tracing::info!("Starting audio engine with model: {}", resolved.display());
    let engine = AudioEngine::start(app, resolved)
        .map_err(|e| format!("Failed to start engine: {e:#}"))?;

    *engine_guard = Some(engine);
    Ok("Audio engine started".into())
}

/// Stop the audio engine.
///
/// Drops the engine, which drops all channel senders, causing
/// all pipeline threads to exit gracefully.
#[tauri::command]
pub async fn stop_audio_engine(
    state: tauri::State<'_, AudioEngineState>,
) -> Result<String, String> {
    let mut engine_guard = state.0.lock().map_err(|e| format!("Lock error: {e}"))?;

    if engine_guard.is_none() {
        return Err("Audio engine is not running".into());
    }

    *engine_guard = None; // Drop the engine → drops channels → threads exit
    Ok("Audio engine stopped".into())
}

/// Toggle microphone processing
#[tauri::command]
pub async fn set_mic_enabled(
    state: tauri::State<'_, AudioEngineState>,
    enabled: bool,
) -> Result<(), String> {
    tracing::info!("set_mic_enabled: {}", enabled);
    let engine_guard = state.0.lock().map_err(|e| format!("Lock error: {e}"))?;
    if let Some(engine) = engine_guard.as_ref() {
        engine.set_mic_enabled(enabled);
    }
    Ok(())
}

/// Toggle system audio processing
#[tauri::command]
pub async fn set_speaker_enabled(
    state: tauri::State<'_, AudioEngineState>,
    enabled: bool,
) -> Result<(), String> {
    tracing::info!("set_speaker_enabled: {}", enabled);
    let engine_guard = state.0.lock().map_err(|e| format!("Lock error: {e}"))?;
    if let Some(engine) = engine_guard.as_ref() {
        engine.set_system_enabled(enabled);
    }
    Ok(())
}
