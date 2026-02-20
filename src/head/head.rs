use crate::errors::AppError;
use bio_utils_rs::io::{get_bufwriter, needletail_reader};
use std::path::PathBuf;

pub fn fasta_head(
    fasta: Option<PathBuf>,
    num_seqs: usize,
    outfile: Option<PathBuf>,
) -> Result<(), AppError> {
    let mut reader = needletail_reader(fasta)?;
    let mut writer = get_bufwriter(outfile)?;

    let mut n: usize = 0;
    while let Some(record) = reader.next() {
        let record = match record {
            Ok(record) => record,
            Err(_) => continue,
        };

        // We only count valid records.
        n += 1;

        record.write(&mut writer, None)?;

        if n >= num_seqs {
            break;
        }
    }

    writer.flush()?;

    Ok(())
}
