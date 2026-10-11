//! Graphics-crate error: wraps the shared core error plus PNG encoding.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] xy_core::Error),

    #[error("png: {0}")]
    Png(#[from] png::EncodingError),
}
