// Make the macro visible in this module
#[macro_use]
mod macros;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::sync::Arc;
// Removed strum imports from here, they are used inside the macro now
use std::str::FromStr; // Needed for infer_format_from_path

// --- Core Enums ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum MediaType {
    Video,
    Audio,
    Image,
    Document,
    // Add other types like Archive if needed
}

// Use the macro to define FileFormat and its impl block
define_file_formats! {
    MediaType::Video => {
        MP4("mp4"), WebM("webm"), GIF("gif"), AVI("avi"), MKV("mkv"),
        WMV("wmv"), MOV("mov"), MTS("mts"), FLV("flv"), OGV("ogv"),
    },
    MediaType::Audio => {
        MP3("mp3"), WAV("wav"), FLAC("flac"), OGG("ogg"), AAC("aac"),
        M4A("m4a"), OPUS("opus"),
    },
    MediaType::Image => {
        JPG("jpg"), JPEG("jpg"), PNG("png"), WEBP("webp"), BMP("bmp"),
        TIFF("tiff"), AVIF("avif"), ICO("ico"),
    },
    MediaType::Document => {
        PDF("pdf"), DOCX("docx"), ODT("odt"), TXT("txt"), HTML("html"),
        MD("md"), EPUB("epub"),
    }
}

// --- Conversion Task & Options ---
// ConversionTask struct remains the same as before...
/// Represents a single conversion operation request.
#[derive(Debug, Clone)]
pub struct ConversionTask {
    pub input_path: PathBuf,
    /// Optional: Explicitly provide source format if it cannot be reliably inferred.
    pub source_format_override: Option<FileFormat>,
    pub output_path: PathBuf,
    pub target_format: FileFormat,
    /// Use Arc<dyn Any + Send + Sync> to hold converter-specific options.
    /// Concrete converters will downcast this to their specific options struct.
    pub options: Option<Arc<dyn Any + Send + Sync>>,
}

impl ConversionTask {
    /// Helper to create a task with options.
    /// The options type `T` must be 'static + Send + Sync.
    pub fn new_with_options<T: Any + Send + Sync>(
        input_path: PathBuf,
        output_path: PathBuf,
        target_format: FileFormat,
        options: T,
        source_format_override: Option<FileFormat>,
    ) -> Self {
        Self {
            input_path,
            source_format_override,
            output_path,
            target_format,
            options: Some(Arc::new(options)),
        }
    }

    /// Helper to create a task without specific options.
    pub fn new(
        input_path: PathBuf,
        output_path: PathBuf,
        target_format: FileFormat,
        source_format_override: Option<FileFormat>,
    ) -> Self {
        Self {
            input_path,
            source_format_override,
            output_path,
            target_format,
            options: None,
        }
    }

    /// Attempts to get the specific options struct of type T.
    pub fn get_options<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
        self.options
            .as_ref()
            .and_then(|opts| opts.clone().downcast::<T>().ok())
    }
}

// --- Converter Trait ---
// Converter trait remains the same as before...
/// Defines the capabilities of any file converter.
/// Use `async_trait` for async methods in traits.
#[async_trait::async_trait]
pub trait Converter: Debug + Send + Sync {
    /// A unique name for this converter (e.g., "FFmpeg Video", "ImageRs Image").
    fn name(&self) -> &'static str;

    /// The primary media type this converter handles.
    fn media_type(&self) -> MediaType;

    /// Returns a list of input formats this converter can handle.
    /// **This is now the sole responsibility of the converter implementation.**
    fn supported_input_formats(&self) -> Vec<FileFormat>;

    /// Returns a list of output formats supported *for a given input format*.
    /// **This is now the sole responsibility of the converter implementation.**
    fn supported_output_formats(&self, input_format: FileFormat) -> Vec<FileFormat>;

    /// Checks if a specific conversion (from -> to) is generally supported by this converter.
    /// Default implementation provided. **This now relies solely on the converter's declared support.**
    fn supports_conversion(&self, from: FileFormat, to: FileFormat) -> bool {
        // Check if 'from' is in the list of supported inputs
        self.supported_input_formats().contains(&from)
            // Check if 'to' is supported *given* 'from'
            && self.supported_output_formats(from).contains(&to)
        // Implicitly, the converter should only list formats of its media type,
        // but we can double-check if necessary (though it adds coupling).
        // && from.media_type() == self.media_type()
        // && to.media_type() == self.media_type() // Let's trust the converter for now
    }

    /// Gets the default file extension for a given output format.
    fn get_output_extension(&self, format: FileFormat) -> &'static str {
        format.default_extension()
    }

    /// Performs the conversion defined by the task.
    async fn convert(&self, task: &ConversionTask) -> Result<()>;
}

// --- Static Converters Holder ---

/// Holds the set of available converters, initialized statically.
#[derive(Debug, Clone)] // Clone is cheap due to Arc
pub struct Converters {
    /// A list of all registered converters. Order might matter for selection.
    available: Vec<Arc<dyn Converter>>,
}

/// Configuration for initializing converters (e.g., paths to executables).
#[derive(Debug, Clone, Default)]
pub struct ConverterConfig {
    pub ffmpeg_path: Option<String>,
    // Add paths or configs for other converters (e.g., ImageMagick, Pandoc)
    // pub imagemagick_path: Option<String>,
}

impl Converters {
    /// Creates a new Converters instance, initializing all known converters.
    /// Requires configuration like paths to external tools.
    pub fn new(config: ConverterConfig) -> Self {
        let mut available: Vec<Arc<dyn Converter>> = Vec::new();

        // --- Initialize Video Converters ---
        // Use configured path or a default ("ffmpeg")
        let ffmpeg_exec = config.ffmpeg_path.unwrap_or_else(|| "ffmpeg".to_string());
        // if Self::command_exists(&ffmpeg_exec) {
        //     // Optional: Check if command exists
        //     let ffmpeg_converter = Arc::new(crate::converters::video::FfmpegVideoConverter::new(
        //         ffmpeg_exec,
        //     ));
        //     available.push(ffmpeg_converter);
        //     log::info!("Initialized FFmpeg video converter.");
        // } else {
        //     log::warn!(
        //         "FFmpeg command ('{}') not found or not configured. Video conversion might be unavailable.",
        //         ffmpeg_exec
        //     );
        // }
        // Add other potential video converters here...

        // --- Initialize Image Converters ---
        // let image_rs_converter = Arc::new(crate::converters::image::ImageRsConverter::new());
        // available.push(image_rs_converter);
        // log::info!("Initialized Image-rs image converter.");
        // Add ImageMagick or others here, potentially checking for existence...

        // --- Initialize Audio Converters ---
        // Example: Could use FFmpeg again if it handles audio formats well
        // Or add a dedicated audio converter (e.g., SoX)

        // --- Initialize Document Converters ---
        // Example: Pandoc, LibreOffice

        Self { available }
    }

    /// Helper to check if a command seems to exist in PATH.
    /// Note: This is a basic check and doesn't guarantee the command works.
    fn command_exists(command: &str) -> bool {
        // which::which(command).is_ok()
        todo!()
    }

    /// Finds the first suitable converter for a given conversion.
    /// It iterates through the statically defined list.
    pub fn find_converter(&self, from: FileFormat, to: FileFormat) -> Option<Arc<dyn Converter>> {
        // Iterate through all available converters
        self.available
            .iter()
            .find(|c| {
                // Check 1: Does the converter handle the *media type* of the input?
                // This avoids asking an image converter about video formats, etc.
                c.media_type() == from.media_type()
                // Check 2: Does the converter explicitly support this specific from -> to conversion?
                && c.supports_conversion(from, to)
            })
            .cloned() // Clone the Arc
    }

    /// Attempts to infer the source format from the file extension.
    fn infer_format_from_path(&self, path: &Path) -> Option<FileFormat> {
        path.extension()
            .and_then(|ext| ext.to_str())
            // Use FromStr trait provided by EnumString
            .and_then(|ext_str| FileFormat::from_str(ext_str.to_lowercase().as_str()).ok())
    }

    /// Performs a conversion using the first available suitable converter.
    pub async fn convert(&self, task: &ConversionTask) -> Result<()> {
        log::debug!("Attempting conversion for task: {:?}", task);

        let source_format = match task.source_format_override {
            Some(fmt) => fmt,
            None => self
                .infer_format_from_path(&task.input_path)
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Could not infer source format from input path: {:?}",
                        task.input_path
                    )
                })?,
        };

        let target_format = task.target_format;

        log::info!(
            "Looking for converter: {} -> {}",
            source_format,
            target_format
        );

        match self.find_converter(source_format, target_format) {
            Some(converter) => {
                log::info!(
                    "Using converter '{}' for {} -> {}",
                    converter.name(),
                    source_format,
                    target_format
                );
                if let Some(parent_dir) = task.output_path.parent() {
                    if !parent_dir.exists() {
                        // tokio::fs::create_dir_all(parent_dir).await?;
                        log::debug!("Created output directory: {:?}", parent_dir);
                    }
                }
                converter.convert(task).await
            }
            None => {
                log::error!(
                    "No suitable converter found for {} -> {}",
                    source_format,
                    target_format
                );
                Err(anyhow::anyhow!(
                    "No suitable converter available for conversion from {} to {}",
                    source_format,
                    target_format
                ))
            }
        }
    }

    /// Get a list of all supported input formats for a given media type across all converters.
    pub fn get_supported_inputs(&self, media_type: MediaType) -> Vec<FileFormat> {
        let mut formats = std::collections::HashSet::new();
        for converter in &self.available {
            if converter.media_type() == media_type {
                formats.extend(converter.supported_input_formats());
            }
        }
        // Sort for consistent UI presentation
        let mut sorted_formats: Vec<_> = formats.into_iter().collect();
        sorted_formats.sort_by_key(|f| f.to_string());
        sorted_formats
    }

    /// Get a list of all possible output formats for a given input format across all converters.
    pub fn get_supported_outputs(&self, input_format: FileFormat) -> Vec<FileFormat> {
        let mut formats = std::collections::HashSet::new();
        let input_media_type = input_format.media_type();

        for converter in &self.available {
            // Only consider converters for the same media type that support the input format
            if converter.media_type() == input_media_type
                && converter.supported_input_formats().contains(&input_format)
            {
                formats.extend(converter.supported_output_formats(input_format));
            }
        }
        // Sort for consistent UI presentation
        let mut sorted_formats: Vec<_> = formats.into_iter().collect();
        sorted_formats.sort_by_key(|f| f.to_string());
        sorted_formats
    }

    /// Get all available converters (useful for diagnostics or advanced selection)
    pub fn get_all_converters(&self) -> &Vec<Arc<dyn Converter>> {
        &self.available
    }
}

// --- Make submodules public ---
// (These remain the same)
// pub mod audio;
// pub mod document;
// pub mod image;
pub mod video;
// pub mod common_options;
