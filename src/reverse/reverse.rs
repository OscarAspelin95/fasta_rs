use needletail::Sequence;

use crate::common::{AppError, get_bufwriter, needletail_fastx_reader};
use std::path::PathBuf;

pub fn fasta_reverse(fasta: Option<PathBuf>, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let mut reader = needletail_fastx_reader(fasta)?;
    let mut writer = get_bufwriter(outfile)?;

    while let Some(record) = reader.next() {
        match record {
            Ok(record) => {
                // Id.
                writer.write_all(b">")?;
                writer.write_all(record.id())?;

                // Rev complement seq.
                writer.write_all(b"\n")?;
                writer.write_all(&record.reverse_complement()[..])?;
                writer.write_all(b"\n")?;
            }
            Err(_) => {
                continue;
            }
        };
    }

    writer.flush()?;

    Ok(())
}
