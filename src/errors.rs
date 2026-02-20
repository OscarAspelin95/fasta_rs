use std::str::Utf8Error;

use bio_utils_rs::errors::BioError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid sample value: {0}")]
    InvalidSampleValueError(f32),

    #[error("Failed to parse primer line: {0}")]
    PrimerLineFormatError(String),

    #[error("Failed to find any primers")]
    NoPrimersFoundError,

    #[error("Invalid range")]
    InvalidRangeError,

    #[error("Invalid regex pattern: {0}")]
    InvalidRegexPattern(String),

    #[error(transparent)]
    BioError(#[from] BioError),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::BioError(BioError::IoError(err))
    }
}

impl From<needletail::errors::ParseError> for AppError {
    fn from(err: needletail::errors::ParseError) -> Self {
        AppError::BioError(BioError::NeedletailParseError(err))
    }
}

impl From<Utf8Error> for AppError {
    fn from(err: Utf8Error) -> Self {
        AppError::BioError(BioError::InvalidParameterError(err.to_string()))
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        AppError::BioError(BioError::InvalidParameterError(err.to_string()))
    }
}

impl From<regex::Error> for AppError {
    fn from(err: regex::Error) -> Self {
        AppError::InvalidRegexPattern(err.to_string())
    }
}
