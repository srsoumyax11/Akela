// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongPtrW, SetWindowLongPtrW, SetWindowDisplayAffinity, GWL_EXSTYLE, WS_EX_TOOLWINDOW, WS_EX_NOACTIVATE,
    WDA_EXCLUDEFROMCAPTURE,
};

mod audio;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize Tracing
    tracing_subscriber::fmt::init();

    let ctrl_shift_a = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyA);

    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if shortcut == &ctrl_shift_a && event.state() == ShortcutState::Pressed {
                        if let Some(window) = app.get_webview_window("main") {
                            let is_visible = window.is_visible().unwrap_or(false);
                            if is_visible {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            let ctrl_shift_a = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyA);
            let _ = app.global_shortcut().register(ctrl_shift_a);

            // Initialize Audio Engine
            let _handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                use crate::audio::audio_events::AudioEvent;
                match audio::init_audio_engine().await {
                    Ok(mut rx) => {
                        tracing::info!("Audio Engine initialized successfully");
                        while let Some(event) = rx.recv().await {
                            if let AudioEvent::ChunkReady(chunk) = event {
                                tracing::debug!("Audio Chunk: {:?} ({} samples)", chunk.source, chunk.samples.len());
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to initialize audio engine: {}", e);
                    }
                }
            });

            // Create Tray Menu
            let show_i = tauri::menu::MenuItem::with_id(app, "show", "Show Akela", true, None::<&str>)?;
            let hide_i = tauri::menu::MenuItem::with_id(app, "hide", "Hide Akela", true, None::<&str>)?;
            let quit_i = tauri::menu::MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = tauri::menu::Menu::with_items(app, &[&show_i, &hide_i, &quit_i])?;

            // Build Tray Icon
            let _tray = tauri::tray::TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let is_visible = window.is_visible().unwrap_or(false);
                            if is_visible {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_skip_taskbar(true);

                // Native Windows: Hide from Alt+Tab by setting WS_EX_TOOLWINDOW
                #[cfg(target_os = "windows")]
                {
                    use raw_window_handle::HasWindowHandle;
                    let handle = window.window_handle().unwrap();
                    if let raw_window_handle::RawWindowHandle::Win32(handle) = handle.as_raw() {
                        let hwnd = HWND(handle.hwnd.get() as *mut core::ffi::c_void);
                        unsafe {
                            let current_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
                            let _ = SetWindowLongPtrW(
                                hwnd,
                                GWL_EXSTYLE,
                                current_style | (WS_EX_TOOLWINDOW.0 as isize) | (WS_EX_NOACTIVATE.0 as isize),
                            );

                            // Stealth Mode: Exclude window from screen capture
                            let _ = SetWindowDisplayAffinity(hwnd, WDA_EXCLUDEFROMCAPTURE);
                        }
                    }
                }

                // Position at top-center
                if let Ok(Some(monitor)) = window.primary_monitor() {
                    let monitor_size = monitor.size();
                    let scale_factor = monitor.scale_factor();
                    let logical_monitor_size = monitor_size.to_logical::<f64>(scale_factor);

                    let window_width = 820.0;
                    let x = (logical_monitor_size.width - window_width) / 2.0;
                    let y = 50.0; // Slightly below the top edge

                    let _ = window.set_position(tauri::LogicalPosition::new(x, y));
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
