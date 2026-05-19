mod window;
mod system;
mod commands;

use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize Tracing
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    system::handle_shortcut(app, shortcut, event);
                })
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            // Register Global Shortcuts
            system::init_shortcuts(app.handle());

            // Create Tray Menu
            system::init_tray(app.handle())?;

            // Setup Main Window
            if let Some(window) = app.get_webview_window("main") {
                window::setup_overlay_window(&window);
            }

            Ok(())
        })
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            commands::greet, 
            commands::hide_window, 
            commands::show_window_no_focus, 
            commands::move_window_no_focus
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
