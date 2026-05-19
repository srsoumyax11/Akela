use tauri::{Window, Manager, AppHandle};
use std::sync::Mutex;
use crate::window;

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

