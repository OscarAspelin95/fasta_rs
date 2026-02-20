use crate::errors::AppError;
use bio::io::fasta::Record;
use bio_utils_rs::io::{bio_fasta_reader, bio_fasta_writer};
use rand::{prelude::*, rng};
use std::path::PathBuf;

pub fn fasta_shuffle(fasta: Option<PathBuf>, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let reader = bio_fasta_reader(fasta)?;
    let mut writer = bio_fasta_writer(outfile)?;

    let mut fasta_records: Vec<Record> =
        reader.records().filter_map(|record| record.ok()).collect();

    // Shuffle records.
    let mut rng = rng();
    fasta_records.shuffle(&mut rng);

    for r in fasta_records {
        writer.write_record(&r)?;
    }

    writer.flush()?;

    Ok(())
}
