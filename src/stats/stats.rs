use crate::common::{AppError, needletail_fastx_reader, write_json};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct FastaStats {
    pub num_seqs: usize,
    pub num_bases: usize,
    pub mean_len: f32,
    pub min_len: usize,
    pub max_len: usize,
}

pub fn fasta_stats(
    fasta: Option<PathBuf>,
    outfile: Option<PathBuf>,
) -> Result<FastaStats, AppError> {
    let mut reader = needletail_fastx_reader(fasta)?;

    let mut num_seqs = 0;
    let mut num_bases = 0;
    let mut min_len: usize = usize::MAX;
    let mut max_len: usize = 0;

    while let Some(record_result) = reader.next() {
        let record = match record_result {
            Ok(record) => record,
            Err(_) => continue,
        };

        let record_len = record.seq().len();

        num_seqs += 1;
        num_bases += record_len;

        min_len = min_len.min(record_len);
        max_len = max_len.max(record_len);
    }

    let fasta_stats = FastaStats {
        num_seqs,
        num_bases,
        mean_len: num_bases as f32 / num_seqs as f32,
        min_len,
        max_len,
    };

    write_json(outfile, &fasta_stats)?;

    Ok(fasta_stats)
}
