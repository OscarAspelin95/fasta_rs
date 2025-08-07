use crate::common::{AppError, needletail_fasta_reader};
use log::warn;
use std::path::PathBuf;

pub fn fasta_extract(
    fasta: &PathBuf,
    start: usize,
    end: usize,
    outfile: &PathBuf,
) -> Result<(), AppError> {
    let mut reader = needletail_fasta_reader(fasta)?;

    if start >= end {
        return Err(AppError::InvalidRangeError);
    }

    while let Some(record) = reader.next() {
        match record {
            Ok(record) => {
                let record_seq = record.seq();

                if start >= record_seq.len() {
                    warn!("Skipping record, too short.");
                    continue;
                }

                let max_end = end.min(record_seq.len());

                let record_id =
                    std::str::from_utf8(record.id()).map_err(|_| AppError::InvalidUtf8Error)?;

                // Add start/end coordinates.
                let record_id_new = format!("{}|{}-{}", record_id, start, max_end);

                println!(
                    ">{}\n{}",
                    record_id_new,
                    std::str::from_utf8(&record.seq()[start..max_end])
                        .expect("Record has invalid UTF-8 encoding.")
                );
            }
            Err(e) => {
                warn!("{:?}", e);
                continue;
            }
        };
    }

    Ok(())
}
