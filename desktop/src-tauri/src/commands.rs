use tauri::{State, WebviewWindow, Wry};
use tokio::sync::RwLock;
use ulid::Ulid;

use crate::converters::{ConversionTask, MediaKind, TaskFilters};
use crate::{WebviewWindowExt, manager::ConversionManager};

type CmdResult<T> = Result<T, String>;
type RwState<'r, T> = State<'r, RwLock<T>>;
type ConversionManagerState<'r> = RwState<'r, ConversionManager>;

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

#[specta::specta]
#[tauri::command]
pub async fn get_tasks(
    state: ConversionManagerState<'_>,
    kind: Option<MediaKind>,
    start: usize,
    limit: usize,
) -> CmdResult<Vec<ConversionTask>> {
    let manager = state.read().await;
    Ok(manager.get_tasks(kind, start, limit))
}

#[specta::specta]
#[tauri::command]
pub async fn get_filtered_tasks(
    state: ConversionManagerState<'_>,
    filters: TaskFilters,
    start: usize,
    limit: usize,
) -> CmdResult<Vec<ConversionTask>> {
    let manager = state.read().await;
    // Ok(manager.get_filtered_tasks(&filters, start, limit))
    todo!()
}

#[specta::specta]
#[tauri::command]
pub async fn start_task(state: ConversionManagerState<'_>, id: Ulid) -> CmdResult<()> {
    let mut manager = state.write().await;
    manager.start_task(&id)
}

#[specta::specta]
#[tauri::command]
pub async fn cancel_task(state: ConversionManagerState<'_>, id: Ulid) -> CmdResult<()> {
    let mut manager = state.write().await;
    manager.cancel_task(&id)
}

#[specta::specta]
#[tauri::command]
pub async fn remove_task(state: ConversionManagerState<'_>, id: Ulid) -> CmdResult<()> {
    let mut manager = state.write().await;
    manager.remove_task(&id)
}

#[specta::specta]
#[tauri::command]
pub async fn clear_tasks(state: ConversionManagerState<'_>) -> CmdResult<()> {
    let mut manager = state.write().await;
    manager.clear();
    Ok(())
}

#[specta::specta]
#[tauri::command]
pub async fn mark_task_visible(state: ConversionManagerState<'_>, id: Ulid) -> CmdResult<()> {
    let mut manager = state.write().await;
    // manager.mark_task_visible(&id);
    todo!()
}

#[specta::specta]
#[tauri::command]
pub async fn mark_task_hidden(state: ConversionManagerState<'_>, id: Ulid) -> CmdResult<()> {
    let mut manager = state.write().await;
    // manager.mark_task_hidden(&id);
    todo!()
}
