use crate::common::{AppError, needletail_fasta_reader};
use std::fs::{create_dir_all, write};
use std::path::PathBuf;

pub fn fasta_split(fasta: &PathBuf, outdir: &PathBuf) -> Result<(), AppError> {
    let mut reader = needletail_fasta_reader(fasta)?;

    create_dir_all(outdir).map_err(|_| AppError::FailedToCreateDirError)?;

    while let Some(record_result) = reader.next() {
        let record = match record_result {
            Ok(record) => record,
            Err(_) => continue,
        };

        // Record ID to string.
        let record_id = std::str::from_utf8(record.id()).map_err(|_| AppError::InvalidUtf8Error)?;

        // Record Seq to string.
        let record_seq = &record.seq();
        let record_seq =
            std::str::from_utf8(&record_seq).map_err(|_| AppError::InvalidUtf8Error)?;

        // Pre-allocate capaticy to string and push contents.
        let mut fasta_contents = String::with_capacity(record_id.len() + 1 + 1 + record_seq.len());
        fasta_contents.push('>');
        fasta_contents.push_str(record_id);
        fasta_contents.push('\n');
        fasta_contents.push_str(record_seq);

        // Define output fasta file.
        let mut fasta_out = outdir.clone();
        fasta_out.push(format!("{record_id}.fasta"));

        // We might need BufWrite of sequences are large.
        write(&fasta_out, fasta_contents).map_err(|_| AppError::FastaWriteError)?;
    }
    Ok(())
}
