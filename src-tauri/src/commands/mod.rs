use tauri::{Window, Manager, AppHandle, State};
use std::sync::Mutex;
use crate::window;
use crate::db::DbState;

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
pub fn resize_window(window: Window, width: i32, height: i32) {
    if let Some(webview) = window.get_webview_window("main") {
        window::resize_window_native(&webview, width, height);
    }
}

#[tauri::command]
pub fn set_window_interactive(window: Window, interactive: bool) {
    if let Some(webview) = window.get_webview_window("main") {
        window::set_window_interactive_native(&webview, interactive);
    }
}

#[tauri::command]
pub fn save_api_key(state: State<'_, DbState>, provider: String, key_value: String) -> Result<(), String> {
    let guard = state.connection.lock().unwrap();
    if let Some(conn) = &*guard {
        conn.execute(
            "INSERT INTO api_keys (provider, key_value) VALUES (?1, ?2)
             ON CONFLICT(provider) DO UPDATE SET key_value = excluded.key_value",
            (&provider, &key_value),
        ).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
pub fn get_api_key(state: State<'_, DbState>, provider: String) -> Result<Option<String>, String> {
    let guard = state.connection.lock().unwrap();
    if let Some(conn) = &*guard {
        let mut stmt = conn.prepare("SELECT key_value FROM api_keys WHERE provider = ?1").map_err(|e| e.to_string())?;
        let mut rows = stmt.query([&provider]).map_err(|e| e.to_string())?;
        if let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let value: String = row.get(0).map_err(|e| e.to_string())?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

