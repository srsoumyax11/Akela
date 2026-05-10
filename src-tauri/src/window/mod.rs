use tauri::WebviewWindow;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongPtrW, SetWindowLongPtrW, SetWindowDisplayAffinity, ShowWindow, SetWindowPos, 
    GWL_EXSTYLE, WS_EX_TOOLWINDOW, WS_EX_NOACTIVATE,
    WDA_EXCLUDEFROMCAPTURE, SW_SHOWNOACTIVATE, SW_HIDE, SWP_NOACTIVATE, SWP_NOSIZE, SWP_NOZORDER,
};

pub fn setup_overlay_window(window: &WebviewWindow) {
    let _ = window.set_skip_taskbar(true);

    #[cfg(target_os = "windows")]
    {
        use raw_window_handle::HasWindowHandle;
        let handle = window.window_handle().unwrap();
        if let raw_window_handle::RawWindowHandle::Win32(handle) = handle.as_raw() {
            let hwnd = HWND(handle.hwnd.get() as *mut core::ffi::c_void);
            unsafe {
                // Set native styles (ToolWindow + NoActivate)
                let current_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
                let _ = SetWindowLongPtrW(
                    hwnd,
                    GWL_EXSTYLE,
                    current_style | (WS_EX_TOOLWINDOW.0 as isize) | (WS_EX_NOACTIVATE.0 as isize),
                );

                // Stealth Mode
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
        let y = 50.0;

        let _ = window.set_position(tauri::LogicalPosition::new(x, y));
    }
}

pub fn show_window_no_focus(window: &WebviewWindow) {
    #[cfg(target_os = "windows")]
    {
        use raw_window_handle::HasWindowHandle;
        if let Ok(handle) = window.window_handle() {
            if let raw_window_handle::RawWindowHandle::Win32(handle) = handle.as_raw() {
                let hwnd = HWND(handle.hwnd.get() as *mut core::ffi::c_void);
                unsafe { let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE); }
                return;
            }
        }
    }
    let _ = window.show();
}

pub fn hide_window_native(window: &WebviewWindow) {
    #[cfg(target_os = "windows")]
    {
        use raw_window_handle::HasWindowHandle;
        if let Ok(handle) = window.window_handle() {
            if let raw_window_handle::RawWindowHandle::Win32(handle) = handle.as_raw() {
                let hwnd = HWND(handle.hwnd.get() as *mut core::ffi::c_void);
                unsafe { let _ = ShowWindow(hwnd, SW_HIDE); }
                return;
            }
        }
    }
    let _ = window.hide();
}

pub fn move_window_native(window: &WebviewWindow, x: i32, y: i32) {
    #[cfg(target_os = "windows")]
    {
        use raw_window_handle::HasWindowHandle;
        if let Ok(handle) = window.window_handle() {
            if let raw_window_handle::RawWindowHandle::Win32(handle) = handle.as_raw() {
                let hwnd = HWND(handle.hwnd.get() as *mut core::ffi::c_void);
                unsafe {
                    let _ = SetWindowPos(
                        hwnd,
                        None,
                        x,
                        y,
                        0,
                        0,
                        SWP_NOACTIVATE | SWP_NOSIZE | SWP_NOZORDER,
                    );
                }
                return;
            }
        }
    }
    let _ = window.set_position(tauri::LogicalPosition::new(x as f64, y as f64));
}
