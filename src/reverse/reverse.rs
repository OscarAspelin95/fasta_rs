use needletail::Sequence;

use crate::common::{AppError, get_bufwriter, needletail_fastx_reader};
use std::path::PathBuf;

pub fn fasta_reverse(fasta: Option<PathBuf>, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let mut reader = needletail_fastx_reader(fasta)?;
    let mut writer = get_bufwriter(outfile).map_err(|_| AppError::FastaWriteError)?;

    while let Some(record) = reader.next() {
        match record {
            Ok(record) => {
                writer
                    .write_all(b">")
                    .map_err(|_| AppError::FastaWriteError)?;

                writer
                    .write_all(record.id())
                    .map_err(|_| AppError::FastaWriteError)?;

                writer
                    .write_all(b"\n")
                    .map_err(|_| AppError::FastaWriteError)?;

                writer
                    .write_all(&record.reverse_complement()[..])
                    .map_err(|_| AppError::FastaWriteError)?;

                writer
                    .write_all(b"\n")
                    .map_err(|_| AppError::FastaWriteError)?;
            }
            Err(_) => {
                continue;
            }
        };
    }

    Ok(())
}
