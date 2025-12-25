use crate::common::writer::bio_fasta_writer;
use crate::common::{AppError, reader::bio_fasta_reader};
use bio::io::fasta::Record;
use rand::{prelude::*, rng};
use std::path::PathBuf;

pub fn fasta_shuffle(fasta: Option<PathBuf>, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let reader = bio_fasta_reader(fasta)?;
    let mut writer = bio_fasta_writer(outfile)?;

    let mut fasta_records: Vec<Record> = Vec::new();

    reader.records().for_each(|record| match record {
        Ok(record) => {
            fasta_records.push(record);
        }
        Err(_) => return,
    });

    // Shuffle records.
    let mut rng = rng();
    fasta_records.shuffle(&mut rng);

    for r in fasta_records {
        writer.write_record(&r)?;
    }

    writer.flush()?;

    Ok(())
}
