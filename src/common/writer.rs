use crate::common::AppError;
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
    let writer = File::create(outfile).map_err(|_| AppError::FastaReadError)?;
    let bufwriter = BufWriter::new(writer);

    return Ok(bufwriter);
}
