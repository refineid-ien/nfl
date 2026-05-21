//! Error types for NFL core operations

use std::io;
use thiserror::Error;

pub type NflResult<T> = Result<T, NflError>;

#[derive(Error, Debug)]
pub enum NflError {
    #[error("Invalid NFL magic bytes: expected {expected:?}, got {actual:?}")]
    InvalidMagic { expected: Vec<u8>, actual: Vec<u8> },

    #[error("Invalid header version: {0}")]
    InvalidVersion(u32),

    #[error("CBOR deserialization error: {0}")]
    CborError(#[from] serde_cbor::Error),

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Invalid tensor index: {0}")]
    InvalidIndex(String),

    #[error("Memory mapping failed: {0}")]
    MmapError(String),

    #[error("Incompatible architecture: {0}")]
    IncompatibleArchitecture(String),

    #[error("File too small: expected at least {expected} bytes, got {actual}")]
    FileTooSmall { expected: usize, actual: usize },

    #[error("Invalid header length: {0}")]
    InvalidHeaderLength(String),

    #[error("Tokenizer not embedded in file")]
    MissingTokenizer,

    #[error("Invalid quantization configuration")]
    InvalidQuantization,

    #[error("Alignment error: address {addr} not aligned to {alignment}")]
    AlignmentError { addr: usize, alignment: usize },

    #[error("Buffer overflow: requested {requested} bytes, available {available}")]
    BufferOverflow { requested: usize, available: usize },

    #[error("{0}")]
    Custom(String),
}

impl NflError {
    /// Get error code for logging/monitoring
    pub fn error_code(&self) -> &'static str {
        match self {
            NflError::InvalidMagic { .. } => "ERR_INVALID_MAGIC",
            NflError::InvalidVersion(_) => "ERR_INVALID_VERSION",
            NflError::CborError(_) => "ERR_CBOR_PARSE",
            NflError::IoError(_) => "ERR_IO",
            NflError::InvalidIndex(_) => "ERR_INVALID_INDEX",
            NflError::MmapError(_) => "ERR_MMAP",
            NflError::IncompatibleArchitecture(_) => "ERR_INCOMPATIBLE_ARCH",
            NflError::FileTooSmall { .. } => "ERR_FILE_TOO_SMALL",
            NflError::InvalidHeaderLength(_) => "ERR_INVALID_HEADER_LEN",
            NflError::MissingTokenizer => "ERR_MISSING_TOKENIZER",
            NflError::InvalidQuantization => "ERR_INVALID_QUANT",
            NflError::AlignmentError { .. } => "ERR_ALIGNMENT",
            NflError::BufferOverflow { .. } => "ERR_BUFFER_OVERFLOW",
            NflError::Custom(_) => "ERR_CUSTOM",
        }
    }

    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            NflError::Custom(_) | NflError::InvalidIndex(_)
        )
    }
}
