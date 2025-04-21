use std::ffi::c_void;

use swift_rs::{Double, swift};
use tauri::{WebviewWindow, Wry};

swift!(
    fn apply_swiftui_window_chrome(nswindow_ptr: *mut c_void, corner_radius: Double)
);

pub fn apply_window_chrome(window: &WebviewWindow<Wry>, corner_radius: f64) -> tauri::Result<()> {
    unsafe {
        apply_swiftui_window_chrome(window.ns_window()?, corner_radius);
    }
    Ok(())
}
