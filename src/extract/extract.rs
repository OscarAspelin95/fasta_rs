use crate::common::{AppError, get_bufwriter, needletail_fastx_reader};
use std::{io::Write, path::PathBuf};

pub fn fasta_extract(
    fasta: Option<PathBuf>,
    start: usize,
    end: usize,
    outfile: Option<PathBuf>,
) -> Result<(), AppError> {
    let mut reader = needletail_fastx_reader(fasta)?;

    if start >= end {
        return Err(AppError::InvalidRangeError.into());
    }

    let mut writer = get_bufwriter(outfile)?;

    while let Some(record) = reader.next() {
        let record = match record {
            Ok(record) => record,
            Err(_) => continue,
        };

        let record_seq = record.seq();

        if start >= record_seq.len() {
            continue;
        }

        let max_end = end.min(record_seq.len());

        let record_id = std::str::from_utf8(record.id())?;

        // Add start/end coordinates.
        let id = format!("{}|{}-{}", record_id, start, max_end);

        // Id.
        writer.write_all(b">")?;
        writer.write_all(id.as_bytes())?;
        writer.write_all(b"\n")?;

        // Sequence.
        writer.write_all(&record.seq()[start..max_end])?;
        writer.write_all(b"\n")?;
    }

    writer.flush()?;

    Ok(())
}
