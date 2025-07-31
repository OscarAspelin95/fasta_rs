use bio::io::fasta::Reader;
use std::fs::File;
use std::{io::BufReader, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FastaParseError {
    #[error("File does not exist")]
    FileDoesNotExistError,

    #[error("Failed to fasta file")]
    FastaReadError,

    #[error("Invalid file extension")]
    InvalidExtensionError,
}

fn validate_fasta(fasta: &PathBuf) -> Result<&PathBuf, FastaParseError> {
    if !fasta.exists() {
        return Err(FastaParseError::FileDoesNotExistError);
    }

    match fasta.extension().and_then(|s| s.to_str()) {
        Some("fasta") | Some("fa") | Some("fsa") | Some("fna") => Ok(fasta),
        _ => Err(FastaParseError::InvalidExtensionError),
    }
}

pub fn fasta_reader(fasta: &PathBuf) -> Result<Reader<BufReader<File>>, FastaParseError> {
    let valid_fasta = validate_fasta(fasta)?;

    let reader = Reader::from_file(valid_fasta).map_err(|_| FastaParseError::FastaReadError)?;
    return Ok(reader);
}
