use serde::{Serialize, ser::Serializer};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error(transparent)]
    Tauri(#[from] tauri::Error),
    // #[error("Stores not found")]
    // StoreNotFound,
    // #[error("Store \"{store_name:?}\" couldn't be saved: {reason:?}")]
    // StoreNotSaved { store_name: String, reason: String },
    // #[error("Invalid app window")]
    // InvalidAppWindow,
    // #[error("Action is not supported on current platform")]
    // PlatformNotSupported,
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl From<Box<dyn core::error::Error + Send + Sync>> for AppError {
    fn from(error: Box<dyn core::error::Error + Send + Sync>) -> Self {
        Self::Anyhow(anyhow::anyhow!(error))
    }
}
