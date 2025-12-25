use crate::common::AppError;
use bio::io::fasta::Reader;
use needletail::{FastxReader, parse_fastx_file, parse_fastx_stdin};
use std::fs::File;
use std::io::Read;
use std::{io::BufReader, path::PathBuf};

const VALID_EXTENSIONS: [&str; 12] = [
    // Non-zipped FASTA.
    ".fasta",
    ".fa",
    ".fna",
    ".fsa",
    // Gzipped FASTA.
    ".fasta.gz",
    ".fa.gz",
    ".fna.gz",
    ".fsa.gz",
    // Non-zipped FASTQ.
    ".fastq",
    ".fq",
    // Gzipped FASTQ.
    ".fastq.gz",
    ".fq.gz",
];

fn validate_fastx(fastx: &PathBuf) -> Result<&PathBuf, AppError> {
    if !fastx.exists() {
        return Err(AppError::FileDoesNotExistError);
    }

    let fastx_str = fastx.to_str().expect("");

    if !VALID_EXTENSIONS
        .iter()
        .any(|extension| fastx_str.ends_with(extension))
    {
        return Err(AppError::InvalidExtensionError(fastx.into()));
    }

    return Ok(fastx);
}

pub fn bio_fasta_reader(
    fasta: Option<PathBuf>,
) -> Result<Reader<BufReader<Box<dyn Read + Send>>>, AppError> {
    match fasta {
        Some(path) => {
            let valid_fasta = validate_fastx(&path)?;
            let f = File::open(valid_fasta)?;

            let reader = Reader::new(Box::new(f) as Box<dyn Read + Send>);
            Ok(reader)
        }
        None => {
            let buf_reader = BufReader::new(std::io::stdin());
            Ok(Reader::new(Box::new(buf_reader)))
        }
    }
}

pub fn needletail_fastx_reader(fastx: Option<PathBuf>) -> Result<Box<dyn FastxReader>, AppError> {
    match fastx {
        Some(fastx_file) => {
            let reader = parse_fastx_file(&validate_fastx(&fastx_file)?)?;

            return Ok(reader);
        }
        None => {
            let reader = parse_fastx_stdin()?;
            return Ok(reader);
        }
    }
}
