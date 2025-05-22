use crate::converters::{ConversionTask, MediaKind, ProgressUpdate};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use ulid::Ulid;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct TaskKey {
    id: Ulid,
    kind: MediaKind,
}

pub struct ConversionManager {
    app_handle: AppHandle,
    tasks: HashMap<TaskKey, Arc<ConversionTask>>,
    active_tasks: HashMap<String, JoinHandle<()>>,
    progress_channels: HashMap<String, mpsc::Sender<ProgressUpdate>>,
    kind_index: HashMap<MediaKind, HashSet<TaskKey>>,
}

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
        let key = TaskKey {
            id,
            kind: task.kind,
        };

        self.tasks.insert(key.clone(), Arc::new(task));
        self.kind_index.entry(key.kind).or_default().insert(key);

        id
    }

    pub fn get_task(&self, id: &Ulid, kind: MediaKind) -> Option<Arc<ConversionTask>> {
        let key = TaskKey { id: *id, kind };
        self.tasks.get(&key).cloned()
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
            .map(|keys: Vec<&TaskKey>| {
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
}
