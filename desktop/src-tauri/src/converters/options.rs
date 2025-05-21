use std::convert::TryFrom;
use std::fmt::Debug;

use crate::converters::FileFormat;
use serde::{Deserialize, Serialize};
use specta::Type;

// Added import for MediaType
use crate::converters::MediaType;
use super::MediaKind; // MediaKind is different from MediaType, keep both for now.

// Define the new trait
pub trait MediaTypeSpecificOptions: Debug + Send + Sync + Clone + Serialize + Type + 'static {
    const MEDIA_TYPE: MediaType;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct VideoConversionOptions {
    /// Encoding speed preset (e.g., "medium", "fast", "slow"). Specific values depend on codec.
    pub speed_preset: Option<String>,
    /// Constant Rate Factor (lower means better quality, larger file). Specific range depends on codec.
    pub crf: Option<u8>,
    /// Target video codec (e.g., "libx264", "libx265", "av1", "copy"). None leaves it to the converter.
    pub video_codec: Option<String>,
    /// Target audio codec (e.g., "aac", "opus", "copy"). None leaves it to the converter.
    pub audio_codec: Option<String>,
    /// Target audio bitrate (e.g., "192k").
    pub audio_bitrate: Option<String>,
    // pub gpu_acceleration: Option<String>, // Example: Could add GPU options later
}

impl MediaTypeSpecificOptions for VideoConversionOptions {
    const MEDIA_TYPE: MediaType = MediaType::Video;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ImageConversionOptions {
    /// Target quality (e.g., 1-100 for lossy formats like JPG/WebP).
    pub quality: Option<u8>,
    /// Optional resizing parameters.
    pub resize: Option<ResizeOptions>,
    /// Format-specific: PNG compression level (0-9).
    pub png_compression: Option<u8>,
    /// Format-specific: WebP lossless encoding.
    pub webp_lossless: Option<bool>,
    // Add other format-specific options as needed
}

impl MediaTypeSpecificOptions for ImageConversionOptions {
    const MEDIA_TYPE: MediaType = MediaType::Image;
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ResizeOptions {
    pub width: Option<u32>,
    pub height: Option<u32>,
    /// How to handle aspect ratio ("stretch", "fit", "fill"). Default should be "fit".
    pub mode: Option<ResizeMode>,
    /// Filter type for resizing (e.g., "lanczos3", "nearest").
    pub filter: Option<String>, // Define enum later if needed
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ResizeMode {
    Stretch, // Ignore aspect ratio
    Fit,     // Fit within dimensions, preserving aspect ratio (default)
    Fill,    // Fill dimensions, preserving aspect ratio (cropping may occur)
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AudioConversionOptions {
    /// Target audio codec (e.g., "mp3", "opus", "flac", "copy").
    pub audio_codec: Option<String>,
    /// Target audio bitrate (e.g., "192k", "320k") or quality level.
    pub audio_quality: Option<String>, // Use string for flexibility (VBR settings etc)
    /// Sample rate (e.g., 44100, 48000).
    pub sample_rate: Option<u32>,
}

impl MediaTypeSpecificOptions for AudioConversionOptions {
    const MEDIA_TYPE: MediaType = MediaType::Audio;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct DocumentConversionOptions {
    /// For PDF output: restrict permissions.
    pub pdf_permissions: Option<PdfPermissions>,
    /// For text output: line wrapping width.
    pub line_wrap: Option<u32>,
    // Add other options specific to document conversions
}

impl MediaTypeSpecificOptions for DocumentConversionOptions {
    const MEDIA_TYPE: MediaType = MediaType::Document;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PdfPermissions {
    pub allow_printing: bool,
    pub allow_copying: bool,
    pub allow_editing: bool,
}

// pub trait ConversionOptions: Debug + Send + Sync + 'static + Default + Serialize + Type {}

// --- Wrapper Enum ---
// This enum will be used in the Tauri command signature and ConversionTask.
// Specta will generate a TypeScript discriminated union from this.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")] // Crucial for TS union
pub enum ConversionOptions {
    Video(VideoConversionOptions),
    Image(ImageConversionOptions),
    Audio(AudioConversionOptions),
    Document(DocumentConversionOptions),
}
// Helper to get the default options for a given media type, wrapped in the enum
// This might be useful within converter implementations or the manager.
impl ConversionOptions {
    pub fn default_for(media_type: crate::converters::MediaKind) -> Self {
        match media_type {
            crate::converters::MediaKind::Video => ConversionOptions::Video(Default::default()),
            crate::converters::MediaKind::Image => ConversionOptions::Image(Default::default()),
            crate::converters::MediaKind::Audio => ConversionOptions::Audio(Default::default()),
            crate::converters::MediaKind::Document => {
                ConversionOptions::Document(Default::default())
            }
        }
    }
}

impl TryFrom<ConversionOptions> for VideoConversionOptions {
    type Error = anyhow::Error;

    fn try_from(value: ConversionOptions) -> Result<Self, Self::Error> {
        match value {
            ConversionOptions::Video(opts) => Ok(opts),
            _ => Err(anyhow::anyhow!(
                "Expected video options but got different type"
            )),
        }
    }
}

impl TryFrom<ConversionOptions> for AudioConversionOptions {
    type Error = anyhow::Error;

    fn try_from(value: ConversionOptions) -> Result<Self, Self::Error> {
        match value {
            ConversionOptions::Audio(opts) => Ok(opts),
            _ => Err(anyhow::anyhow!(
                "Expected audio options but got different type"
            )),
        }
    }
}

impl TryFrom<ConversionOptions> for ImageConversionOptions {
    type Error = anyhow::Error;

    fn try_from(value: ConversionOptions) -> Result<Self, Self::Error> {
        match value {
            ConversionOptions::Image(opts) => Ok(opts),
            _ => Err(anyhow::anyhow!(
                "Expected image options but got different type"
            )),
        }
    }
}

impl TryFrom<ConversionOptions> for DocumentConversionOptions {
    type Error = anyhow::Error;

    fn try_from(value: ConversionOptions) -> Result<Self, Self::Error> {
        match value {
            ConversionOptions::Document(opts) => Ok(opts),
            _ => Err(anyhow::anyhow!(
                "Expected document options but got different type"
            )),
        }
    }
}
