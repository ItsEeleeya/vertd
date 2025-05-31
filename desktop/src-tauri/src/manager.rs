use crate::converters::error::ConverterError;
use crate::converters::{ConversionTask, ConverterResult, FileFormat, MediaKind, ProgressUpdate};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use tauri::AppHandle;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use ulid::Ulid;

pub struct ConversionManager {
    app_handle: AppHandle,
    tasks: VecDeque<ConversionTask>,
    idx: HashMap<Ulid, usize>, // Map ID to position in VecDeque
    kind_index: HashMap<MediaKind, HashSet<Ulid>>,
    active_tasks: HashMap<Ulid, JoinHandle<()>>,
    progress_channels: HashMap<String, mpsc::Sender<ProgressUpdate>>,
}

impl ConversionManager {
    pub fn new(app_handle: &AppHandle) -> Self {
        Self {
            app_handle: app_handle.clone(),
            tasks: VecDeque::with_capacity(15),
            idx: HashMap::new(),
            active_tasks: HashMap::new(),
            progress_channels: HashMap::new(),
            kind_index: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, task: ConversionTask) -> Ulid {
        let id = task.id;
        let kind = task.kind;
        let pos = self.tasks.len();
        self.tasks.push_back(task);
        self.idx.insert(id, pos);
        self.kind_index.entry(kind).or_default().insert(id);
        id
    }

    pub fn add_task_from_path(&mut self, path: &Path) -> ConverterResult<()> {
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| ConverterError::InvalidFileExtension(path.into()))?;

        let source_format = FileFormat::try_from(extension)
            .map_err(|_| ConverterError::UnsupportedFileFormat(extension.into()))?;

        let kind = source_format.media_kind();

        let task = ConversionTask::new(kind, path.to_path_buf(), None, source_format, None, None);
        self.add_task(task);
        Ok(())
    }

    pub fn get_task(&self, id: &Ulid) -> Option<&ConversionTask> {
        self.idx.get(id).and_then(|&pos| self.tasks.get(pos))
    }

    pub fn print_all(&self) {
        println!("Tasks ------ ");
        dbg!(&self.tasks);
        println!("Kind Index - ");
        dbg!(&self.kind_index);
    }

    pub fn get_tasks(
        &self,
        kind: Option<MediaKind>,
        start: usize,
        limit: usize,
    ) -> Vec<ConversionTask> {
        let tasks = match kind {
            Some(kind) => self
                .kind_index
                .get(&kind)
                .map(|keys| {
                    self.tasks
                        .iter()
                        .filter(|task| keys.contains(&task.id))
                        .cloned()
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
            None => self.tasks.iter().cloned().collect::<Vec<_>>(),
        };

        tasks.into_iter().skip(start).take(limit).collect()
    }

    pub fn get_task_count(&self, kind: Option<MediaKind>) -> usize {
        match kind {
            Some(kind) => self.kind_index.get(&kind).map_or(0, |keys| keys.len()),
            None => self.tasks.len(),
        }
    }

    pub fn clear(&mut self) {
        self.tasks.clear();
        self.idx.clear();
        self.active_tasks.clear();
        self.progress_channels.clear();
        self.kind_index.clear();
    }

    // Task state management
    pub fn start_task(&mut self, id: &Ulid) -> Result<(), String> {
        todo!()
    }

    pub fn cancel_task(&mut self, id: &Ulid) -> Result<(), String> {
        todo!()
    }

    pub fn remove_task(&mut self, id: &Ulid) -> Result<(), String> {
        todo!()
    }

    // Progress channel management
    pub fn register_progress_channel(
        &mut self,
        id: &Ulid,
        kind: MediaKind,
        sender: mpsc::Sender<ProgressUpdate>,
    ) -> Result<(), String> {
        todo!()
    }

    pub fn remove_progress_channel(&mut self, id: &Ulid) -> Result<(), String> {
        todo!()
    }

    // Task status queries
    pub fn is_task_active(&self, id: &Ulid, kind: MediaKind) -> bool {
        todo!()
    }

    pub fn get_task_progress(&self, id: &Ulid, kind: MediaKind) -> Option<ProgressUpdate> {
        todo!()
    }

    pub fn get_active_tasks(&self) -> Vec<ConversionTask> {
        todo!()
    }
}
