use crate::errors::AppError;
use bio_utils_rs::io::{get_bufwriter, needletail_reader};
use regex::Regex;
use std::path::PathBuf;

pub fn fasta_grep(
    fastq: Option<PathBuf>,
    pattern: String,
    outfile: Option<PathBuf>,
) -> Result<(), AppError> {
    let mut reader = needletail_reader(fastq)?;
    let mut writer = get_bufwriter(outfile)?;

    let pattern = Regex::new(pattern.as_str())?;

    while let Some(record) = reader.next() {
        let record = match record {
            Ok(record) => record,
            Err(_) => continue,
        };

        match pattern.captures(std::str::from_utf8(record.id())?) {
            Some(_) => record.write(&mut writer, None)?,
            None => continue,
        }
    }

    Ok(())
}
