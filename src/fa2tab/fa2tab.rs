use crate::common::{AppError, gc_content, get_bufwriter, needletail_fasta_reader};
use std::{io::Write, path::PathBuf};

pub fn fasta_fa2tab(fasta: &PathBuf, outfile: &PathBuf) -> Result<(), AppError> {
    let mut reader = needletail_fasta_reader(fasta)?;

    let mut bufwriter = get_bufwriter(outfile)?;

    while let Some(record_result) = reader.next() {
        let record = match record_result {
            Ok(record) => record,
            Err(_) => continue,
        };

        let record_seq = record.seq();
        let record_id = record.id();
        let record_len = record_seq.len();
        let gc_content: f32 = gc_content(&record_seq);

        let tab_stat = format!(
            "{}\t{}\t{}\n",
            std::str::from_utf8(record_id).expect("Invalid UTF-8 encoding for record ID."),
            record_len,
            gc_content,
        );

        bufwriter.write_all(tab_stat.as_bytes()).unwrap();
    }

    Ok(())
}
