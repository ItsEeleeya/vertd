use crate::converters::progress::{ProgressDetails, ProgressUpdate, VideoProgressDetails};
use crate::converters::{
    ConversionTask, Converter, FileFormat, MediaKind,
    options::{ConversionOptions, VideoConversionOptions},
};
use anyhow::{Context, Result, anyhow};
use log::{debug, error, info, trace, warn};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration as StdDuration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio::time::{Duration as TokioDuration, Instant};

use super::ConverterResult;

const VIDEO_INPUT_FORMATS: &[FileFormat] = &[
    FileFormat::Mp4,
    FileFormat::Webm,
    FileFormat::Avi,
    FileFormat::Mkv,
    FileFormat::Wmv,
    FileFormat::Mov,
    FileFormat::Mts,
    FileFormat::Flv,
    FileFormat::Ogv,
    FileFormat::Gif,
];

const VIDEO_OUTPUT_FORMATS: &[FileFormat] = &[
    FileFormat::Mp4,
    FileFormat::Webm,
    FileFormat::Avi,
    FileFormat::Mkv,
    FileFormat::Mov,
    FileFormat::Gif,
];

#[derive(Debug, Clone)]
pub struct VideoConverter {
    ffmpeg_path: PathBuf,
    ffprobe_path: PathBuf,
    throttle_duration: TokioDuration,
}

impl VideoConverter {
    pub fn new() -> Self {
        Self {
            ffmpeg_path: "ffmpeg".into(),
            ffprobe_path: "ffprobe".into(),
            throttle_duration: TokioDuration::from_millis(250),
        }
    }

    // Helper to parse ffmpeg's progress output line (key=value format)
    fn parse_ffmpeg_progress(line: &str, details: &mut VideoProgressDetails) -> bool {
        todo!()
    }

    // Function to estimate total duration using ffprobe
    async fn estimate_duration_seconds_static(
        ffprobe_path: PathBuf,
        input_path: PathBuf,
    ) -> Result<Option<f64>> {
        todo!()
    }

    // Helper to calculate percentage based on time processed and total duration
    fn calculate_percentage(
        details: &VideoProgressDetails,
        total_duration_secs: Option<f64>,
    ) -> f32 {
        match (total_duration_secs, &details.time_processed) {
            (Some(total_secs), Some(time_str)) if total_secs > 0.0 => {
                // Try parsing "secs.fractional" format first
                if let Some(secs_part) = time_str.split('.').next() {
                    if let Ok(processed_secs) = secs_part.parse::<f64>() {
                        return (processed_secs * 100.0 / total_secs).clamp(0.0, 100.0) as f32;
                    }
                }
                // Fallback: Try parsing HH:MM:SS strings using humantime (requires humantime crate)
                // if let Ok(parsed_duration) = humantime::parse_duration(time_str) {
                //     let processed_secs = parsed_duration.as_secs_f64();
                //     return (processed_secs * 100.0 / total_secs).clamp(0.0, 100.0);
                // }

                warn!(
                    "Could not parse time_processed '{}' for percentage calculation.",
                    time_str
                );
                0.0 // Cannot parse time string reliably
            }
            _ => 0.0, // Cannot calculate percentage without duration or valid time
        }
    }
}

#[async_trait::async_trait]
impl Converter for VideoConverter {
    fn name(&self) -> &'static str {
        "FFmpeg Video Converter"
    }

    fn media_type(&self) -> MediaKind {
        MediaKind::Video
    }

    fn supported_input_formats(&self) -> &[FileFormat] {
        VIDEO_INPUT_FORMATS
    }

    fn supported_output_formats(&self, _input_format: FileFormat) -> &[FileFormat] {
        VIDEO_OUTPUT_FORMATS
    }

    async fn convert(
        &self,
        task: Arc<ConversionTask>,
    ) -> ConverterResult<mpsc::Receiver<ProgressUpdate>> {
        let options = task.get_typed_options::<VideoConversionOptions>()?;

        todo!()
    }
}
