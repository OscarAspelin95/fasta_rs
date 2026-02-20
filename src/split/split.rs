use crate::errors::AppError;
use bio_utils_rs::io::{get_bufwriter, needletail_reader};
use std::fs::create_dir_all;
use std::path::PathBuf;

pub fn fasta_split(fasta: Option<PathBuf>, outdir: &PathBuf) -> Result<(), AppError> {
    let mut reader = needletail_reader(fasta)?;

    create_dir_all(outdir)?;

    while let Some(record_result) = reader.next() {
        let record = match record_result {
            Ok(record) => record,
            Err(_) => continue,
        };

        // Define output fasta file.
        let outfile = outdir.join(format!("{}.fasta", std::str::from_utf8(record.id())?));

        let mut writer = get_bufwriter(Some(outfile))?;
        record.write(&mut writer, None)?;

        writer.flush()?;
    }

    Ok(())
}
