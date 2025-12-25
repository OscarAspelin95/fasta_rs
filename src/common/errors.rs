use std::{path::PathBuf, str::Utf8Error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid UTF-8: {0}")]
    InvalidUtf8Error(#[from] Utf8Error),

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

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid integer.")]
    ParseError(#[from] std::num::ParseIntError),

    #[error("Invalid fastx file format.")]
    InvalidFastxFormatError(#[from] needletail::errors::ParseError),

    #[error("Invalid regex pattern.")]
    InvalidRegexPattern(#[from] regex::Error),
}
