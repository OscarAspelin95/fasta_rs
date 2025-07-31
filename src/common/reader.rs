use crate::common::AppError;
use bio::io::fasta::Reader;
use needletail::{FastxReader, parse_fastx_file};
use std::fs::File;
use std::{io::BufReader, path::PathBuf};

fn validate_fasta(fasta: &PathBuf) -> Result<&PathBuf, AppError> {
    if !fasta.exists() {
        return Err(AppError::FileDoesNotExistError);
    }

    match fasta.extension().and_then(|s| s.to_str()) {
        Some("fasta") | Some("fa") | Some("fsa") | Some("fna") => Ok(fasta),
        _ => Err(AppError::InvalidExtensionError),
    }
}

pub fn bio_fasta_reader(fasta: &PathBuf) -> Result<Reader<BufReader<File>>, AppError> {
    let valid_fasta = validate_fasta(fasta)?;

    let reader = Reader::from_file(valid_fasta).map_err(|_| AppError::FastaReadError)?;
    return Ok(reader);
}

pub fn needletail_fasta_reader(fasta: &PathBuf) -> Result<Box<dyn FastxReader>, AppError> {
    let valid_fasta = validate_fasta(fasta)?;

    let reader = parse_fastx_file(&valid_fasta).map_err(|_| AppError::FastaReadError)?;
    return Ok(reader);
}
