use crate::errors::AppError;
use bio_utils_rs::io::{get_bufwriter, needletail_reader};
use std::{fs::create_dir_all, path::PathBuf};

pub fn fasta_chunk(
    fasta: Option<PathBuf>,
    num_contigs_per_file: usize,
    outdir: &PathBuf,
) -> Result<(), AppError> {
    if num_contigs_per_file == 0 {
        return Err(AppError::InvalidArgError(
            "`--num-contigs-per-file must be non-zero`".into(),
        ));
    }

    let mut reader = needletail_reader(fasta)?;
    create_dir_all(outdir)?;

    let mut count: usize = 0;
    let mut chunk: usize = 0;
    let mut writer = None;

    while let Some(record_result) = reader.next() {
        let record = match record_result {
            Ok(r) => r,
            Err(_) => continue,
        };

        if count.is_multiple_of(num_contigs_per_file) {
            chunk += 1;
            let path = outdir.join(format!("chunk_{chunk}.fasta"));
            writer = Some(get_bufwriter(Some(path))?);
        }

        record.write(writer.as_mut().expect("unexpected writer error"), None)?;
        count += 1;
    }

    Ok(())
}
