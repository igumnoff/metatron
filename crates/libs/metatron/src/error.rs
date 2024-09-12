use thiserror::Error;

/// Error type for the `metatron-core` crate.
#[derive(Error, Debug)]
pub enum ReportError {
    #[error("Common error: {0}")]
    Common(String),

    #[error("Invalid document type: {0}")]
    InvalidDocumentType(String),

    #[error("KDL parse error: {0}")]
    KdlParseError(#[from] kdl::KdlError),

    #[error("JSON parse error: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Float parse error: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
}
