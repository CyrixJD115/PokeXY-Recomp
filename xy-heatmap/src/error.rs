//! Shared error type — the only exception-like failure channel (port of
//! `errors.py`). Every expected failure (bad data root, malformed dataset,
//! unknown module) is a `HeatmapError`; the CLI prints `error: …` and exits 1.

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct HeatmapError {
    pub message: String,
}

impl HeatmapError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl From<std::io::Error> for HeatmapError {
    fn from(e: std::io::Error) -> Self {
        Self::new(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, HeatmapError>;
