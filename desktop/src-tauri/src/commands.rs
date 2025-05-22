use tauri::{WebviewWindow, Wry};

use crate::WebviewWindowExt;

type CmdResult<T> = Result<T, String>;

#[specta::specta]
#[tauri::command]
pub fn invalidate_shadow(window: WebviewWindow<Wry>) -> CmdResult<()> {
    #[cfg(target_os = "macos")]
    unsafe {
        window
            .objc2_nswindow()
            .map_err(|e| e.to_string())?
            .invalidateShadow();
    }
    Ok(())
}

// A function to create and add tasks based on a list of strings, each being a file path
#[specta::specta]
#[tauri::command]
pub fn fetch_and_add_tasks(window: WebviewWindow<Wry>, file_paths: Vec<String>) -> CmdResult<()> {
    Ok(())
}
