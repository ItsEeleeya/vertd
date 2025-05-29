use std::path::Path;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::{State, WebviewWindow, Wry};
use tokio::sync::RwLock;
use ulid::Ulid;

use crate::converters::FileFormat;
use crate::converters::{ConversionTask, MediaKind, TaskFilters};
use crate::{WebviewWindowExt, manager::ConversionManager};

type CmdResult<T> = std::result::Result<T, String>;
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
pub async fn add_tasks(
    manager: ConversionManagerState<'_>,
    file_paths: Vec<String>,
) -> CmdResult<()> {
    let mut writer = manager.write().await;
    for path in file_paths {
        if let Err(e) = writer.add_task_from_path(Path::new(&path)) {
            eprintln!("Failed to add task for {path}: {e}");
        }
    }
    Ok(())
}

#[specta::specta]
#[tauri::command]
pub async fn get_tasks(
    manager: ConversionManagerState<'_>,
    kind: Option<MediaKind>,
    start: f64,
    limit: f64,
) -> CmdResult<Vec<ConversionTask>> {
    let manager = manager.read().await;
    println!("Getting tasks at: {start}, limit: {limit}");
    Ok(manager.get_tasks(kind, start as usize, limit as usize))
}

#[specta::specta]
#[tauri::command]
pub async fn get_filtered_tasks(
    manager: ConversionManagerState<'_>,
    filters: TaskFilters,
    start: usize,
    limit: usize,
) -> CmdResult<Vec<ConversionTask>> {
    let manager = manager.read().await;
    // Ok(manager.get_filtered_tasks(&filters, start, limit))
    todo!()
}

#[specta::specta]
#[tauri::command]
pub async fn start_task(manager: ConversionManagerState<'_>, id: Ulid) -> CmdResult<()> {
    let mut manager = manager.write().await;
    manager.start_task(&id)
}

#[specta::specta]
#[tauri::command]
pub async fn cancel_task(manager: ConversionManagerState<'_>, id: Ulid) -> CmdResult<()> {
    let mut manager = manager.write().await;
    manager.cancel_task(&id)
}

#[specta::specta]
#[tauri::command]
pub async fn remove_task(manager: ConversionManagerState<'_>, id: Ulid) -> CmdResult<()> {
    let mut manager = manager.write().await;
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
pub async fn mark_task_visible(manager: ConversionManagerState<'_>, id: Ulid) -> CmdResult<()> {
    // let mut manager = manager.write().await;
    // manager.mark_task_visible(&id);
    todo!()
}

#[specta::specta]
#[tauri::command]
pub async fn mark_task_hidden(manager: ConversionManagerState<'_>, id: Ulid) -> CmdResult<()> {
    // let mut manager = manager.write().await;
    // manager.mark_task_hidden(&id);
    todo!()
}

#[specta::specta]
#[tauri::command]
pub async fn add_mock_tasks(manager: ConversionManagerState<'_>, count: f64) -> CmdResult<()> {
    let mut writer = manager.write().await;
    let kinds = [
        MediaKind::Video,
        MediaKind::Audio,
        MediaKind::Image,
        MediaKind::Document,
    ];

    for i in 0..(count as u64) {
        let kind = kinds[i as usize % kinds.len()];
        let format = match kind {
            MediaKind::Video => FileFormat::Mp4,
            MediaKind::Audio => FileFormat::Mp3,
            MediaKind::Image => FileFormat::Png,
            MediaKind::Document => FileFormat::Pdf,
        };
        let input_path = PathBuf::from(format!(
            "/mock/path/file_{}.{}",
            i,
            format.default_extension()
        ));

        let task = ConversionTask::new(kind, input_path, None, format, None, None);
        writer.add_task(task);
    }
    Ok(())
}

#[specta::specta]
#[tauri::command]
pub async fn subscribe_to_progress(
    app: AppHandle,
    manager: ConversionManagerState<'_>,
) -> CmdResult<()> {
    // let mut manager = manager.write().await;
    // let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    // Store the sender in the manager
    // manager.register_progress_channel(tx)?;

    // Spawn a task to forward progress updates to the frontend
    // tokio::spawn(async move {
    //     while let Some(update) = rx.recv().await {
    //         if let Err(e) = app.emit_all("progress-update", update) {
    //             error!("Failed to emit progress update: {}", e);
    //         }
    //     }
    // });

    Ok(())
}
