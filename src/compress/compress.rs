use crate::common::{AppError, get_bufwriter, needletail_fastx_reader};
use log::{info, warn};
use rstest::*;

use std::{io::Write, path::PathBuf};

fn homopolymer_compression(seq: &[u8], max_hp_len: usize) -> Vec<u8> {
    let mut hp_compressed: Vec<u8> = Vec::with_capacity(seq.len());

    let mut i: usize = 0;

    while i < seq.len() {
        let mut j = i + 1;

        while j < seq.len() && seq[j] == seq[i] {
            j += 1;
        }

        for _ in 0..std::cmp::min(j - i, max_hp_len) {
            hp_compressed.push(seq[i]);
        }
        i = j;
    }

    hp_compressed
}

pub fn fasta_compress(
    fasta: Option<PathBuf>,
    max_hp_len: usize,
    outfile: &PathBuf,
) -> Result<(), AppError> {
    assert!(max_hp_len > 0, "value of max_hp_len must be > 0.");

    let mut reader = needletail_fastx_reader(fasta)?;

    // Output file writer.
    let mut bufwriter = get_bufwriter(outfile)?;

    info!("Finding homopolymers...");
    while let Some(record) = reader.next() {
        match record {
            Ok(record) => {
                let compressed_sequence = homopolymer_compression(&record.seq(), max_hp_len);

                let s: String = format!(
                    ">{}\n{}\n",
                    std::str::from_utf8(record.id()).map_err(|_| AppError::InvalidUtf8Error)?,
                    std::str::from_utf8(&compressed_sequence[..])
                        .map_err(|_| AppError::InvalidUtf8Error)?
                );
                bufwriter
                    .write_all(s.as_bytes())
                    .map_err(|_| AppError::FastaWriteError)?
            }
            Err(e) => {
                warn!("Failed to parse record: {:?}", e);
            }
        }
    }
    Ok(())
}

#[rstest]
#[case(b"", 1, b"")]
#[case(b"ATCG", 1, b"ATCG")]
#[case(b"ATCG", 2, b"ATCG")]
#[case(b"AAAAA", 3, b"AAA")]
#[case(b"AAATTTCCCGGG", 2, b"AATTCCGG")]
#[case(b"AAATTTCCCGGG", 3, b"AAATTTCCCGGG")]
#[case(b"AAATTTCCCGGG", 10, b"AAATTTCCCGGG")]

fn test_compression(#[case] seq: &[u8], #[case] max_hp_len: usize, #[case] expected_seq: &[u8]) {
    assert_eq!(&homopolymer_compression(seq, max_hp_len)[..], expected_seq);
}
