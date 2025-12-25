use crate::common::AppError;
use crate::common::reader::bio_fasta_reader;
use crate::common::writer::bio_fasta_writer;
use bio::io::fasta::Record;
use rand::{prelude::*, rng};
use std::path::PathBuf;

pub fn fasta_sample(
    fasta: Option<PathBuf>,
    by: f32,
    outfile: Option<PathBuf>,
) -> Result<(), AppError> {
    let reader = bio_fasta_reader(fasta)?;
    let mut writer = bio_fasta_writer(outfile)?;

    let mut fasta_records: Vec<Record> = Vec::new();

    reader.records().for_each(|record| match record {
        Ok(record) => {
            fasta_records.push(record);
        }
        Err(_) => return,
    });

    // Randomly choose records to keep.
    let sample_by;

    if by <= 0.0 {
        return Err(AppError::InvalidSampleValueError(by).into());
    }

    if by <= 1.0 {
        sample_by = (by * fasta_records.len() as f32) as usize;
    } else {
        sample_by = by as usize;
    }

    let mut rng = rng();
    let sample = fasta_records.choose_multiple(&mut rng, sample_by);

    for r in sample {
        writer.write_record(r)?;
    }

    writer.flush()?;

    Ok(())
}
