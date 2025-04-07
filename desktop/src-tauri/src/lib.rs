use tauri::{Manager, WebviewWindow};
use tauri_plugin_window_state::StateFlags;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .with_state_flags(StateFlags::SIZE | StateFlags::POSITION | StateFlags::MAXIMIZED)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.webview_windows()
                .iter()
                .try_for_each(|(_, window)| unsafe {
                    let nswindow = window.objc2_nswindow()?;
                    nswindow.setOpaque(true);
                    nswindow.setCollectionBehavior(
                        objc2_app_kit::NSWindowCollectionBehavior::FullScreenNone,
                    );
                    Ok::<(), tauri::Error>(())
                })?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(target_os = "macos")]
pub trait WebviewWindowExt {
    fn objc2_nswindow(&self) -> tauri::Result<&objc2_app_kit::NSWindow>;
}

#[cfg(target_os = "macos")]
impl WebviewWindowExt for WebviewWindow {
    #[inline]
    fn objc2_nswindow(&self) -> tauri::Result<&objc2_app_kit::NSWindow> {
        // SAFETY: This cast is safe as the existence of the WebviewWindow means it's attached to an NSWindow
        unsafe { Ok(&*self.ns_window()?.cast()) }
    }
}
