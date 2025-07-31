use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("File does not exist")]
    FileDoesNotExistError,

    #[error("Failed to fasta file")]
    FastaReadError,

    #[error("Invalid file extension")]
    InvalidExtensionError,

    #[error("Failed to parse primer file.")]
    PrimerFileParsingError,

    #[error("Failed to parse primer line.")]
    PrimerLineFormatError,

    #[error("Invalid primer length.")]
    PrimerLenParsingError,

    #[error("Failed to find any primers.")]
    NoPrimersFoundError,
}
