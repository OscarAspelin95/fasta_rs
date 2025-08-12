use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid UTF-8")]
    InvalidUtf8Error,

    #[error("Could not create directory")]
    FailedToCreateDirError,

    #[error("File does not exist")]
    FileDoesNotExistError,

    #[error("Failed to read fasta file")]
    FastxReadError,

    #[error("Failed to write to fasta file")]
    FastaWriteError,

    #[error("Invalid file extension")]
    InvalidExtensionError,

    // Amplicon Primer Errors.
    #[error("Failed to parse primer file.")]
    PrimerFileParsingError,

    #[error("Failed to parse primer line.")]
    PrimerLineFormatError,

    #[error("Invalid primer length.")]
    PrimerLenParsingError,

    #[error("Failed to find any primers.")]
    NoPrimersFoundError,

    #[error("Invalid range.")]
    InvalidRangeError,

    #[error("Invalid sample value")]
    InvalidSampleValueError,
}
