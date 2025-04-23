use error::AppError;
use tauri::{Manager, WebviewWindow};
use tauri_plugin_window_state::StateFlags;

mod commands;
mod error;
mod platform;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let specta_builder = tauri_specta::Builder::<tauri::Wry>::new()
        .events(tauri_specta::collect_events!())
        .commands(tauri_specta::collect_commands![commands::invalidate_shadow]);

    #[cfg(debug_assertions)]
    specta_builder
        .export(
            specta_typescript::Typescript::default()
                .formatter(specta_typescript::formatter::prettier)
                .header("// @ts-nocheck"),
            "../src/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .with_state_flags(StateFlags::SIZE | StateFlags::POSITION | StateFlags::MAXIMIZED)
                .build(),
        )
        .plugin(
            tauri_plugin_prevent_default::Builder::default()
                .with_flags({
                    use tauri_plugin_prevent_default::Flags;
                    #[cfg(debug_assertions)]
                    {
                        Flags::all()
                            .difference(Flags::DEV_TOOLS | Flags::RELOAD | Flags::CONTEXT_MENU)
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        Flags::all()
                    }
                })
                .build(),
        )
        .invoke_handler(specta_builder.invoke_handler())
        .setup(move |app| {
            specta_builder.mount_events(app);

            // TODO: Add an option for using default decoration on macOS.
            #[cfg(target_os = "macos")]
            app.webview_windows()
                .iter()
                .try_for_each(|(_, window)| unsafe {
                    window.objc2_nswindow()?.setCollectionBehavior(
                        objc2_app_kit::NSWindowCollectionBehavior::FullScreenNone,
                    );
                    platform::apply_window_chrome(window, 20.0)
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
