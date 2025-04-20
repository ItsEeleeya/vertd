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
