use serde::Serialize;
use thiserror::Error;

// TODO: clean up error handling
#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("login failed: {0}")]
    LoginError(String),
    // #[error("other")]
    // Other(),
    // #[error("http client error: {0}")]
    // HttpError(#[from] reqwest::Error),

    // #[error("sync error: {0}")]
    // SyncError(#[from] tokio::sync::oneshot::error::RecvError),

    // #[error("json error")]
    // JsonError(#[from] serde_json::Error),

    // #[error(transparent)]
    // Other(#[from] anyhow::Error),

    // #[error(transparent)]
    // TimeError(#[from] std::time::SystemTimeError),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
