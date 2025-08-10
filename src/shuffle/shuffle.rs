use crate::common::reader::bio_fasta_reader;
use crate::common::writer::get_fasta_writer;
use crate::common::AppError;
use bio::io::fasta::Record;
use log::info;
use rand::{prelude::*, rng};
use std::path::PathBuf;

pub fn fasta_shuffle(fasta: &PathBuf, outfile: &PathBuf) -> Result<(), AppError> {
    let reader = bio_fasta_reader(fasta)?;

    let mut writer = get_fasta_writer(&outfile)?;

    info!("Parsing records...");
    let mut fasta_records: Vec<Record> = Vec::new();

    reader.records().for_each(|record| match record {
        Ok(record) => {
            fasta_records.push(record);
        }
        Err(_) => return,
    });

    info!("Shuffling records...");
    let mut rng = rng();
    fasta_records.shuffle(&mut rng);

    info!("Writing records...");
    for r in fasta_records {
        writer
            .write_record(&r)
            .map_err(|_| AppError::FastaWriteError)?;
    }

    Ok(())
}
