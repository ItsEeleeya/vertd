use crate::converters::{ConversionTask, MediaKind, ProgressUpdate};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use ulid::Ulid;

struct VisibleTasks {
    ids: VecDeque<Ulid>,
    capacity: usize,
}

impl VisibleTasks {
    fn new(capacity: usize) -> Self {
        Self {
            ids: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    fn add(&mut self, id: Ulid) {
        if self.ids.len() >= self.capacity {
            self.ids.pop_front();
        }
        self.ids.push_back(id);
    }

    fn remove(&mut self, id: &Ulid) {
        self.ids.retain(|&x| x != *id);
    }

    fn contains(&self, id: &Ulid) -> bool {
        self.ids.contains(id)
    }

    fn is_full(&self) -> bool {
        self.ids.len() >= self.capacity
    }
}

pub struct ConversionManager {
    app_handle: AppHandle,
    tasks: HashMap<Ulid, Arc<ConversionTask>>,
    active_tasks: HashMap<Ulid, JoinHandle<()>>,
    progress_channels: HashMap<String, mpsc::Sender<ProgressUpdate>>,
    kind_index: HashMap<MediaKind, HashSet<Ulid>>,
}

// #[derive(Debug, Clone, Hash, Eq, PartialEq)]
// struct TaskKey {
//     id: Ulid,
//     kind: MediaKind,
// }

impl ConversionManager {
    pub fn new(app_handle: &AppHandle) -> Self {
        Self {
            app_handle: app_handle.clone(),
            tasks: HashMap::new(),
            active_tasks: HashMap::new(),
            progress_channels: HashMap::new(),
            kind_index: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, task: ConversionTask) -> Ulid {
        let id = task.id;
        let kind = task.kind;
        self.tasks.insert(id, Arc::new(task));
        self.kind_index.entry(kind).or_default().insert(id);
        id
    }

    pub fn get_task(&self, id: &Ulid) -> Option<Arc<ConversionTask>> {
        self.tasks.get(id).cloned()
    }

    pub fn get_tasks(
        &self,
        kind: Option<MediaKind>,
        start: usize,
        limit: usize,
    ) -> Vec<ConversionTask> {
        let task_keys = match kind {
            Some(kind) => self
                .kind_index
                .get(&kind)
                .map(|keys| keys.iter().collect::<Vec<_>>()),
            None => Some(self.tasks.keys().collect::<Vec<_>>()),
        };

        task_keys
            .map(|keys| {
                keys.iter()
                    .skip(start)
                    .take(limit)
                    .filter_map(|key| self.tasks.get(key).cloned())
                    .map(|task| (*task).clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn get_task_count(&self, kind: Option<MediaKind>) -> usize {
        match kind {
            Some(kind) => self.kind_index.get(&kind).map_or(0, |keys| keys.len()),
            None => self.tasks.len(),
        }
    }

    pub fn clear(&mut self) {
        self.tasks.clear();
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

    // Batch operations
    pub fn get_all_tasks(&self, kind: Option<MediaKind>) -> Vec<ConversionTask> {
        todo!()
    }

    pub fn get_active_tasks(&self) -> Vec<ConversionTask> {
        todo!()
    }
}
