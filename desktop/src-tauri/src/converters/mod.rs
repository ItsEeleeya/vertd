#![allow(unused)]
#[macro_use]
mod macros;
mod options;
mod progress;
mod video;

use options::{
    AudioConversionOptions, ConversionOptions, DocumentConversionOptions, ImageConversionOptions,
    VideoConversionOptions,
};
pub use progress::{
    AudioProgressDetails, DocumentProgressDetails, ImageProgressDetails, ProgressDetails,
    ProgressUpdate, VideoProgressDetails,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use strum::{Display, EnumString};
use tokio::sync::mpsc;
use video::VideoConverter;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, Serialize, Deserialize, Type,
)]
#[strum(serialize_all = "lowercase")]
pub enum MediaKind {
    Video,
    Audio,
    Image,
    Document,
}

define_file_formats! {
    MediaKind::Video => {
        MP4("mp4"), WebM("webm"), Gif("gif"), AVI("avi"), MKV("mkv"),
        WMV("wmv"), MOV("mov"), MTS("mts"), FLV("flv"), OGV("ogv"),
    },
    MediaKind::Audio => {
        MP3("mp3"), WAV("wav"), FLAC("flac"), OGG("ogg"), AAC("aac"),
        M4A("m4a"), OPUS("opus"),
    },
    MediaKind::Image => {
        JPG("jpg"), JPEG("jpg"), PNG("png"), WEBP("webp"), BMP("bmp"),
        TIFF("tiff"), AVIF("avif"), ICO("ico"),
    },
    MediaKind::Document => {
        PDF("pdf"), DOCX("docx"), ODT("odt"), TXT("txt"), HTML("html"),
        MD("md"), EPUB("epub"),
    }
}

pub enum Converters {
    Video { converter: VideoConverter },
}

#[derive(Debug, Clone)]
pub struct ConversionTask {
    pub id: String,
    pub kind: MediaKind,
    pub input_path: PathBuf,
    pub source_format_override: Option<FileFormat>,
    pub output_path: PathBuf,
    pub target_format: FileFormat,
    pub options: Option<ConversionOptions>,
}

impl ConversionTask {
    pub fn new_with_options(
        id: String,
        kind: MediaKind,
        input_path: PathBuf,
        output_path: PathBuf,
        target_format: FileFormat,
        options: ConversionOptions,
        source_format_override: Option<FileFormat>,
    ) -> Self {
        Self {
            id,
            kind,
            input_path,
            source_format_override,
            output_path,
            target_format,
            options: Some(options),
        }
    }

    pub fn new(
        id: String,
        kind: MediaKind,
        input_path: PathBuf,
        output_path: PathBuf,
        target_format: FileFormat,
        source_format_override: Option<FileFormat>,
    ) -> Self {
        Self {
            id,
            kind,
            input_path,
            source_format_override,
            output_path,
            target_format,
            options: None,
        }
    }

    /// Gets the conversion options for this task, returning default options if none were provided.
    pub fn get_options(&self) -> ConversionOptions {
        self.options
            .clone()
            .unwrap_or_else(|| ConversionOptions::default_for(self.kind))
    }

    /// Gets the conversion options of the specified type, returning an error if the options don't match.
    ///
    /// # Example
    /// ```rust
    /// let video_opts = task.get_typed_options::<VideoConversionOptions>()?;
    /// ```
    pub fn get_typed_options<T>(&self) -> anyhow::Result<T>
    where
        T: TryFrom<ConversionOptions, Error = anyhow::Error>,
    {
        self.get_options().try_into()
    }

    /// Ensures that the task's options match its media kind.
    pub fn validate_options(&self) -> anyhow::Result<()> {
        match (self.kind, &self.options) {
            (MediaKind::Video, Some(ConversionOptions::Video(_))) => Ok(()),
            (MediaKind::Audio, Some(ConversionOptions::Audio(_))) => Ok(()),
            (MediaKind::Image, Some(ConversionOptions::Image(_))) => Ok(()),
            (MediaKind::Document, Some(ConversionOptions::Document(_))) => Ok(()),
            (_, None) => Ok(()), // No options is valid
            (kind, Some(opts)) => Err(anyhow::anyhow!(
                "Media kind {:?} does not match options type {:?}",
                kind,
                opts
            )),
        }
    }
}

#[async_trait::async_trait]
pub trait Converter: Debug + Send + Sync {
    fn name(&self) -> &'static str;
    fn media_type(&self) -> MediaKind;
    fn supported_input_formats(&self) -> &[FileFormat];
    fn supported_output_formats(&self, input_format: FileFormat) -> &[FileFormat];
    fn supports_conversion(&self, from: FileFormat, to: FileFormat) -> bool {
        self.supported_input_formats().contains(&from)
            && self.supported_output_formats(from).contains(&to)
    }
    fn get_output_extension(&self, format: FileFormat) -> &'static str {
        format.default_extension()
    }

    async fn convert(&self, task: Arc<ConversionTask>) -> Result<mpsc::Receiver<ProgressUpdate>>;
}

// pub struct Manager {
//     converters: HashMap<MediaType, Vec<Arc<dyn Converter>>>,
// }

// impl Manager {
//     pub fn register(&mut self, converter: Arc<dyn Converter>) {
//         self.converters
//             .entry(converter.media_type())
//             .or_default()
//             .push(converter);
//     }

//     pub fn find_converter_for_task(&self, task: &ConversionTask) -> Option<Arc<dyn Converter>> {
//         let target_media_type = task.target_format.media_type();
//         let source_format = task.source_format_override.or_else(|| {
//             task.input_path
//                 .extension()
//                 .and_then(|ext| ext.to_str())
//                 .and_then(|ext_str| FileFormat::from_str(ext_str).ok())
//         })?;

//         self.converters
//             .get(&target_media_type)?
//             .iter()
//             .find(|c| c.supports_conversion(source_format, task.target_format))
//             .cloned()
//     }
// }
