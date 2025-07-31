use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("File does not exist")]
    FileDoesNotExistError,

    #[error("Failed to fasta file")]
    FastaReadError,

    #[error("Invalid file extension")]
    InvalidExtensionError,
}
