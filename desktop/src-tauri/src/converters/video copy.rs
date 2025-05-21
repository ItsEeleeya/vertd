use crate::converters::progress::{ProgressDetails, ProgressUpdate, VideoProgressDetails};
use crate::converters::{
    ConversionTask, Converter, FileFormat, MediaType,
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

const VIDEO_INPUT_FORMATS: &[FileFormat] = &[
    FileFormat::MP4,
    FileFormat::WebM,
    FileFormat::AVI,
    FileFormat::MKV,
    FileFormat::WMV,
    FileFormat::MOV,
    FileFormat::MTS,
    FileFormat::FLV,
    FileFormat::OGV,
    FileFormat::GIF,
];

const VIDEO_OUTPUT_FORMATS: &[FileFormat] = &[
    FileFormat::MP4,
    FileFormat::WebM,
    FileFormat::AVI,
    FileFormat::MKV,
    FileFormat::MOV,
    FileFormat::GIF,
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
        trace!("Parsing ffmpeg progress line: {}", line);
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            match key {
                "frame" => details.frame = value.parse().ok(),
                "fps" => details.fps = value.parse().ok(),
                "bitrate" => {
                    details.bitrate_kbit =
                        value.trim_end_matches("kbits/s").trim().parse::<f64>().ok();
                }
                "total_size" => {
                    details.size_kb = value.parse::<u64>().ok().map(|b| b / 1024);
                }
                "out_time_us" => {
                    if details.time_processed.is_none() {
                        if let Ok(us) = value.parse::<u64>() {
                            let secs = us / 1_000_000;
                            let micros = us % 1_000_000;
                            details.time_processed = Some(format!("{}.{:06}", secs, micros));
                        }
                    }
                }
                "out_time_ms" => {
                    if let Ok(ms) = value.parse::<u64>() {
                        let secs = ms / 1000;
                        let millis = ms % 1000;
                        details.time_processed = Some(format!("{}.{:03}", secs, millis));
                    }
                }
                "out_time" => {
                    if details.time_processed.is_none() {
                        details.time_processed = Some(value.to_string());
                    }
                }
                "speed" => {
                    details.speed = value.trim_end_matches('x').trim().parse().ok();
                }
                "progress" => {
                    if value == "end" {
                        debug!("FFmpeg progress signal 'end' received.");
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    // Function to estimate total duration using ffprobe
    async fn estimate_duration_seconds_static(
        ffprobe_path: PathBuf,
        input_path: PathBuf,
    ) -> Result<Option<f64>> {
        info!("Estimating duration for: {:?}", input_path);
        // *** MODIFIED: Use the passed ffprobe_path ***
        let mut cmd = Command::new(&ffprobe_path);
        cmd.arg("-v")
            .arg("error")
            .arg("-show_entries")
            .arg("format=duration")
            .arg("-of")
            .arg("default=noprint_wrappers=1:nokey=1")
            // *** MODIFIED: Use the passed input_path ***
            .arg(input_path.as_os_str())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let output = cmd
            .output()
            .await
            .with_context(|| format!("Failed to execute ffprobe for {:?}", input_path))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("ffprobe failed for {:?}: {}", input_path, stderr);
            return Err(anyhow!("ffprobe failed: {}", stderr));
        }

        let duration_str = String::from_utf8(output.stdout)
            .context("ffprobe output was not valid UTF-8")?
            .trim()
            .to_string();

        if duration_str.is_empty() || duration_str == "N/A" {
            debug!("ffprobe could not determine duration for {:?}", input_path);
            Ok(None)
        } else {
            match duration_str.parse::<f64>() {
                Ok(duration) if duration > 0.0 => {
                    info!(
                        "Estimated duration for {:?}: {} seconds",
                        input_path, duration
                    );
                    Ok(Some(duration))
                }
                _ => {
                    error!(
                        "ffprobe returned invalid duration '{}' for {:?}",
                        duration_str, input_path
                    );
                    Ok(None) // Treat invalid parse as unknown duration
                }
            }
        }
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
impl Converter<VideoConversionOptions> for VideoConverter {
    fn name(&self) -> &'static str {
        "FFmpeg Video Converter"
    }

    fn media_type(&self) -> MediaType {
        MediaType::Video
    }

    fn supported_input_formats(&self) -> &[FileFormat] {
        VIDEO_INPUT_FORMATS
    }

    fn supported_output_formats(&self, _input_format: FileFormat) -> &[FileFormat] {
        VIDEO_OUTPUT_FORMATS
    }

    async fn convert(&self, task: Arc<ConversionTask>) -> Result<mpsc::Receiver<ProgressUpdate>> {
        // --- Task Setup ---
        let task_id = task.id.clone();
        let input_path = task.input_path.clone();
        let output_path = task.output_path.clone();
        info!(
            "Starting conversion task [{}] from {:?} to {:?}",
            task_id, input_path, output_path
        );

        if let Some(parent) = output_path.parent() {
            tokio::fs::create_dir_all(parent).await.with_context(|| {
                format!(
                    "Task [{}] Failed to create output directory: {:?}",
                    task_id, parent
                )
            })?;
        }

        let options = task.options.as_ref().unwrap();

        // --- Pre-computation (Duration Estimation) ---
        let ffprobe_path_clone = self.ffprobe_path.clone();
        let input_path_clone = input_path.clone();

        let duration_handle = tokio::spawn(async move {
            Self::estimate_duration_seconds_static(ffprobe_path_clone, input_path_clone).await
        });

        // --- Build FFmpeg Command ---
        let mut args: Vec<String> = vec![
            "-hide_banner".to_string(),
            "-loglevel".to_string(),
            "error".to_string(),
            "-progress".to_string(),
            "pipe:1".to_string(),
            "-y".to_string(),
            "-i".to_string(),
            input_path.to_string_lossy().to_string(),
        ];

        // Use the specific VideoConversionOptions type
        if let Some(codec) = &options.video_codec {
            args.push("-c:v".to_string());
            args.push(codec.clone());
        }
        if let Some(preset) = &options.speed_preset {
            args.push("-preset".to_string());
            args.push(preset.clone());
        }
        if let Some(crf) = options.crf {
            args.push("-crf".to_string());
            args.push(crf.to_string());
        }
        if let Some(audio_codec) = &options.audio_codec {
            args.push("-c:a".to_string());
            args.push(audio_codec.clone());
        }
        if let Some(audio_bitrate) = &options.audio_bitrate {
            args.push("-b:a".to_string());
            args.push(audio_bitrate.clone());
        }

        args.push(output_path.to_string_lossy().to_string());

        // --- Wait for Duration Estimation ---
        let total_duration_secs: Option<f64> = match duration_handle.await {
            // Match block remains the same
            Ok(Ok(Some(duration))) => {
                info!(
                    "Task [{}] Estimated duration: {} seconds",
                    task_id, duration
                );
                Some(duration)
            }
            Ok(Ok(None)) => {
                warn!(
                    "Task [{}] Could not determine duration for percentage.",
                    task_id
                );
                None
            }
            Ok(Err(e)) => {
                error!("Task [{}] Failed to estimate duration: {}", task_id, e);
                None
            }
            Err(e) => {
                error!(
                    "Task [{}] Duration estimation task panicked: {}",
                    task_id, e
                );
                None
            }
        };

        // --- Spawn FFmpeg Process ---
        info!(
            "Task [{}]: Spawning {} with args: {}",
            task_id,
            self.ffmpeg_path.display(),
            args.join(" ")
        );

        let mut command = Command::new(&self.ffmpeg_path);
        command
            .args(&args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = command
            .spawn()
            .context(format!("Task [{}] Failed to spawn ffmpeg process", task_id))?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow!("Task [{}] Failed to capture ffmpeg stdout", task_id))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| anyhow!("Task [{}] Failed to capture ffmpeg stderr", task_id))?;

        let (tx, rx) = mpsc::channel::<ProgressUpdate>(32);

        // --- Task to Handle Stdout (Progress) ---
        let tx_stdout = tx.clone();
        let task_id_stdout = task_id.clone();
        let throttle_duration = self.throttle_duration;

        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            let mut progress_details_state = VideoProgressDetails {
                estimated_duration_secs: total_duration_secs, // Store estimate
                ..Default::default()
            };

            let mut last_update_time = Instant::now();
            let mut end_signal_received = false;

            // Send initial "Starting" update
            let initial_update = ProgressUpdate::with_details(
                task_id_stdout.clone(),
                0.0,
                Some("Starting".to_string()),
                ProgressDetails::Video(progress_details_state.clone()),
            );
            if tx_stdout.send(initial_update).await.is_err() {
                debug!(
                    "Task [{}]: Progress receiver dropped (initial).",
                    task_id_stdout
                );
                return;
            }
            last_update_time = Instant::now();

            while let Ok(Some(line)) = lines.next_line().await {
                let mut current_details = progress_details_state.clone();
                let is_end_signal = Self::parse_ffmpeg_progress(&line, &mut current_details);

                if !is_end_signal {
                    progress_details_state = current_details;
                } else {
                    end_signal_received = true;
                }

                let percentage =
                    Self::calculate_percentage(&progress_details_state, total_duration_secs);

                let now = Instant::now();
                if now.duration_since(last_update_time) >= throttle_duration || end_signal_received
                {
                    let update = ProgressUpdate::with_details(
                        task_id_stdout.clone(),
                        percentage,
                        Some("Encoding".to_string()),
                        ProgressDetails::Video(progress_details_state.clone()),
                    );

                    if tx_stdout.send(update).await.is_err() {
                        debug!(
                            "Task [{}]: Progress receiver dropped (stdout loop).",
                            task_id_stdout
                        );
                        break;
                    }
                    last_update_time = now;
                }

                if end_signal_received {
                    break;
                }
            }
            debug!("Task [{}]: Stdout processing finished.", task_id_stdout);
        });

        // --- Task to Handle Stderr (Errors/Logs) ---
        let tx_stderr = tx.clone();
        let task_id_stderr = task_id.clone();
        tokio::spawn(async move {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                error!("Task [{}]: FFmpeg stderr: {}", task_id_stderr, line);
                let update = ProgressUpdate::new_error(task_id_stderr.clone(), line);
                if tx_stderr.send(update).await.is_err() {
                    debug!(
                        "Task [{}]: Progress receiver dropped (stderr loop).",
                        task_id_stderr
                    );
                    break;
                }
            }
            debug!("Task [{}]: Stderr processing finished.", task_id_stderr);
        });

        // --- Task to Wait for Process Exit and Send Final Update ---
        let task_id_final = task_id.clone();
        tokio::spawn(async move {
            match child.wait().await {
                Ok(status) => {
                    if status.success() {
                        info!(
                            "Task [{}]: FFmpeg process completed successfully.",
                            &task_id_final
                        );
                        let update = ProgressUpdate::new_done(task_id_final.clone());
                        if tx.send(update).await.is_err() {
                            debug!(
                                "Task [{}]: Progress receiver dropped before final success update.",
                                &task_id_final
                            );
                        }
                    } else {
                        let id = task_id_final.clone();
                        error!(
                            "Task [{}]: FFmpeg process failed with status: {}",
                            task_id_final, status
                        );
                        let update = ProgressUpdate::new_error(
                            id,
                            format!("FFmpeg failed with status: {}", status),
                        );
                        if tx.send(update).await.is_err() {
                            debug!(
                                "Task [{}]: Progress receiver dropped before final error update.",
                                &task_id_final
                            );
                        }
                    }
                }
                Err(e) => {
                    error!(
                        "Task [{}]: Failed to wait for FFmpeg process: {}",
                        &task_id_final, e
                    );
                    let update = ProgressUpdate::new_error(
                        task_id_final.clone(),
                        format!("Failed to wait for FFmpeg process: {}", e),
                    );
                    if tx.send(update).await.is_err() {
                        debug!(
                            "Task [{}]: Progress receiver dropped before wait error update.",
                            &task_id_final
                        );
                    }
                }
            }
            debug!("Task [{}]: Final status task finished.", task_id_final);
        });

        info!(
            "Task [{}] FFmpeg process spawned and listeners attached.",
            task_id
        );
        Ok(rx)
    }
}
