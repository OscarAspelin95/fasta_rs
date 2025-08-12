use crate::common::{AppError, get_bufwriter, needletail_fastx_reader};
use std::path::PathBuf;

pub fn fasta_head(
    fasta: Option<PathBuf>,
    num_seqs: usize,
    outfile: Option<PathBuf>,
) -> Result<(), AppError> {
    let mut reader = needletail_fastx_reader(fasta)?;
    let mut writer = get_bufwriter(outfile).map_err(|_| AppError::FastaWriteError)?;

    let mut n: usize = 0;
    while let Some(record) = reader.next() {
        match record {
            Ok(record) => {
                n += 1;

                record
                    .write(&mut writer, None)
                    .map_err(|_| AppError::FastaWriteError)?
            }
            Err(_) => {
                continue;
            }
        };

        if n >= num_seqs {
            break;
        }
    }

    Ok(())
}
