use crate::errors::AppError;
use bio::io::fasta::Record;
use bio_utils_rs::io::{bio_fasta_reader, bio_fasta_writer};
use rand::{prelude::*, rng};
use std::path::PathBuf;

pub fn fasta_sample(
    fasta: Option<PathBuf>,
    by: f32,
    outfile: Option<PathBuf>,
) -> Result<(), AppError> {
    let reader = bio_fasta_reader(fasta)?;
    let mut writer = bio_fasta_writer(outfile)?;

    let fasta_records: Vec<Record> = reader.records().filter_map(|record| record.ok()).collect();

    // Randomly choose records to keep.

    if by <= 0.0 {
        return Err(AppError::InvalidSampleValueError(by));
    }

    let sample_by = if by <= 1.0 {
        (by * fasta_records.len() as f32) as usize
    } else {
        by as usize
    };

    let mut rng = rng();
    let sample = fasta_records.choose_multiple(&mut rng, sample_by);

    for r in sample {
        writer.write_record(r)?;
    }

    writer.flush()?;

    Ok(())
}
