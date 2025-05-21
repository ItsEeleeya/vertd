use serde::{Deserialize, Serialize};
use specta::Type;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct VideoProgressDetails {
    pub frame: Option<u64>,
    pub fps: Option<f32>,
    pub bitrate_kbit: Option<f64>,
    pub size_kb: Option<u64>,
    pub time_processed: Option<String>, // e.g., "00:01:23.45" or seconds string
    pub speed: Option<f32>,             // e.g., 1.5x
    // Add estimated total duration maybe?
    pub estimated_duration_secs: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AudioProgressDetails {
    // pub time_processed: Option<String>,
    // pub size_kb: Option<u64>,
    // pub speed: Option<f32>,
    // pub bitrate_kbit: Option<f64>,
    // pub estimated_duration_secs: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ImageProgressDetails {
    // // Images might finish quickly, maybe just step names?
    // pub current_step: Option<String>, // e.g., "Decoding", "Resizing", "Encoding"
    // // or maybe percentage if multi-frame (GIF) or complex processing
    // pub percentage: Option<f32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct DocumentProgressDetails {
    // pub pages_processed: Option<f32>,
    // pub total_pages: Option<f32>,
    // pub current_step: Option<String>, // maybe "Parsing", "Rendering", "Saving"
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum ProgressDetails {
    Video(VideoProgressDetails),
    Audio(AudioProgressDetails),
    Image(ImageProgressDetails),
    Document(DocumentProgressDetails),
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum ProgressStatus {
    Done,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ProgressUpdate {
    pub task_id: String,
    #[serde(default)]
    pub percentage: f32,
    pub status: Option<ProgressStatus>,
    pub status_message: Option<String>,
    pub details: Option<ProgressDetails>,
}

impl ProgressUpdate {
    pub fn with_details(
        task_id: String,
        percentage: f32,
        status: Option<String>,
        details: ProgressDetails,
    ) -> Self {
        Self {
            task_id,
            percentage,
            status: None,
            status_message: status,
            details: Some(details),
        }
    }

    pub fn new_error(task_id: String, error_message: String) -> Self {
        Self {
            task_id,
            percentage: 0.0,
            status: Some(ProgressStatus::Failed),
            status_message: Some(error_message),
            details: None,
        }
    }

    pub fn new_done(task_id: String) -> Self {
        Self {
            task_id,
            percentage: 100.0,
            status: Some(ProgressStatus::Done),
            status_message: Some("Completed".to_string()),
            details: None,
        }
    }

    pub fn update_percentage(&mut self, percentage: f32) {
        self.percentage = percentage;
    }

    pub fn is_completed(&self) -> bool {
        matches!(self.status, Some(ProgressStatus::Done))
    }

    pub fn is_failed(&self) -> bool {
        matches!(self.status, Some(ProgressStatus::Failed))
    }

    pub fn update_status_message(&mut self, message: String) {
        self.status_message = Some(message);
    }
}
