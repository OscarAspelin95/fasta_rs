use crate::common::fasta_reader;
use log::warn;
use std::path::PathBuf;

pub fn fasta_head(fasta: &PathBuf, num_seqs: usize) {
    let reader = fasta_reader(fasta).unwrap();

    let mut records = reader.records();

    let mut n: usize = 0;
    while let Some(record) = records.next() {
        match record {
            Ok(record) => {
                n += 1;

                println!(
                    ">{}\n{}",
                    record.id(),
                    std::str::from_utf8(record.seq()).unwrap()
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
}
