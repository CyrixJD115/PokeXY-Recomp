//! Error type shared by all parsers.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("bad magic at {offset:#x}: expected {expected}, found {found}")]
    Magic {
        offset: usize,
        expected: &'static str,
        found: String,
    },

    #[error("truncated: need {needed} byte(s) at {offset:#x} ({context})")]
    Truncated {
        offset: usize,
        needed: usize,
        context: &'static str,
    },

    #[error("invalid {context}: {details}")]
    Invalid {
        context: &'static str,
        details: String,
    },

    #[error("unsupported {context}: {details}")]
    Unsupported {
        context: &'static str,
        details: String,
    },
}

impl Error {
    pub fn magic(offset: usize, expected: &'static str, found: &[u8]) -> Self {
        let found = found
            .iter()
            .map(|b| format!("{b:02X}"))
            .collect::<Vec<_>>()
            .join(" ");
        Self::Magic {
            offset,
            expected,
            found,
        }
    }

    pub fn invalid(context: &'static str, details: impl Into<String>) -> Self {
        Self::Invalid {
            context,
            details: details.into(),
        }
    }

    pub fn unsupported(context: &'static str, details: impl Into<String>) -> Self {
        Self::Unsupported {
            context,
            details: details.into(),
        }
    }

    pub fn truncated(offset: usize, needed: usize, context: &'static str) -> Self {
        Self::Truncated {
            offset,
            needed,
            context,
        }
    }
}
