use crate::common::{AppError, needletail_fastx_reader};
use log::warn;
use std::path::PathBuf;

pub fn fasta_head(fasta: Option<PathBuf>, num_seqs: usize) -> Result<(), AppError> {
    let mut reader = needletail_fastx_reader(fasta)?;

    let mut n: usize = 0;
    while let Some(record) = reader.next() {
        match record {
            Ok(record) => {
                n += 1;

                println!(
                    ">{}\n{}",
                    std::str::from_utf8(record.id())
                        .expect("Record ID has invalid UTF-8 encoding."),
                    std::str::from_utf8(&record.seq()).expect("Record has invalid UTF-8 encoding.")
                );
            }
            Err(e) => {
                warn!("{:?}", e);
                continue;
            }
        };

        if n >= num_seqs {
            break;
        }
    }

    Ok(())
}
