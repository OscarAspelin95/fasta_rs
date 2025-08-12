use crate::common::AppError;
use bio::io::fasta::Writer;
use serde::Serialize;
use serde_json;
use std::path::PathBuf;
use std::{fs::File, io::BufWriter};

pub fn write_json<T: Serialize>(outfile: &PathBuf, s: T) {
    let outbuf = File::create(outfile).unwrap();
    let writer = BufWriter::new(outbuf);

    serde_json::to_writer(writer, &s).unwrap();
}

pub fn get_bufwriter(outfile: &PathBuf) -> Result<BufWriter<File>, AppError> {
    let writer = File::create(outfile).map_err(|_| AppError::FastxReadError)?;
    let bufwriter = BufWriter::new(writer);

    return Ok(bufwriter);
}

// We might need BufWriter here...
pub fn get_fasta_writer(outfile: &PathBuf) -> Result<Writer<BufWriter<File>>, AppError> {
    let writer = Writer::new(BufWriter::new(
        File::create(outfile).map_err(|_| AppError::FastxReadError)?,
    ));

    return Ok(writer);
}
