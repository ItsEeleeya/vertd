use crate::converters::{FileFormat, MediaKind};
use std::io;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum ConverterError {
    #[error("Input file not found: {0}")]
    InputFileNotFound(PathBuf),

    #[error("Output error for path: {path}")]
    OutputError {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("(Internal) Invalid conversion options: {0}")]
    InternalInvalidOptions(String),

    #[error("Mismatched media kind for {context}: Expected {expected}")]
    MismatchedMediaKind {
        expected: MediaKind,
        context: String,
    },

    #[error(
        "Target format {target_format:?} (kind: {format_kind:?}) is incompatible with task kind {task_kind:?}"
    )]
    MismatchedTargetFormat {
        task_kind: MediaKind,
        target_format: FileFormat,
        format_kind: MediaKind,
    },

    #[error("Failed to execute tool '{tool}': {source}")]
    ToolExecutionError {
        tool: String,
        #[source]
        source: io::Error,
    },

    #[error("Error parsing output from tool '{tool}': {message}")]
    ToolOutputParseError { tool: String, message: String },

    #[error("Unsupported conversion from {from:?} to {to:?}")]
    UnsupportedConversion { from: FileFormat, to: FileFormat },

    #[error("I/O error: {0}")]
    IO(#[from] io::Error),

    #[error("Internal converter error: {0}")]
    Internal(String),

    #[error("Failed to send progress update: {0}")]
    ProgressChannelSendError(String),

    #[error("Custom error: {0}")]
    Custom(String),
}

impl From<anyhow::Error> for ConverterError {
    fn from(err: anyhow::Error) -> Self {
        ConverterError::Custom(err.to_string())
    }
}
