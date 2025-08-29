use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid UTF-8: {0}")]
    InvalidUtf8Error(PathBuf),

    #[error("File does not exist")]
    FileDoesNotExistError,

    #[error("Invalid file extension for: {0}")]
    InvalidExtensionError(PathBuf),

    #[error("Failed to parse primer line.")]
    PrimerLineFormatError(String),

    #[error("Failed to find any primers.")]
    NoPrimersFoundError,

    #[error("Invalid range.")]
    InvalidRangeError,

    #[error("Invalid sample value: {0}")]
    InvalidSampleValueError(f32),
}
