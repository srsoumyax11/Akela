use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use crate::window;

pub fn init_shortcuts(app: &AppHandle) {
    let ctrl_shift_a = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyA);
    let _ = app.global_shortcut().register(ctrl_shift_a);
}

pub fn handle_shortcut(app: &AppHandle, shortcut: &Shortcut, event: tauri_plugin_global_shortcut::ShortcutEvent) {
    let ctrl_shift_a = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyA);
    
    if shortcut == &ctrl_shift_a && event.state() == ShortcutState::Pressed {
        if let Some(window) = app.get_webview_window("main") {
            let is_visible = window.is_visible().unwrap_or(false);
            if is_visible {
                window::hide_window_native(&window);
            } else {
                window::show_window_no_focus(&window);
            }
        }
    }
}

pub fn init_tray(app: &AppHandle) -> tauri::Result<()> {
    let show_i = tauri::menu::MenuItem::with_id(app, "show", "Show Akela", true, None::<&str>)?;
    let hide_i = tauri::menu::MenuItem::with_id(app, "hide", "Hide Akela", true, None::<&str>)?;
    let quit_i = tauri::menu::MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = tauri::menu::Menu::with_items(app, &[&show_i, &hide_i, &quit_i])?;

    let _tray = tauri::tray::TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    window::show_window_no_focus(&window);
                }
            }
            "hide" => {
                if let Some(window) = app.get_webview_window("main") {
                    window::hide_window_native(&window);
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
                        window::hide_window_native(&window);
                    } else {
                        window::show_window_no_focus(&window);
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}
