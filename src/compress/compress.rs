use crate::common::{AppError, get_bufwriter, needletail_fastx_reader};
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
    outfile: Option<PathBuf>,
) -> Result<(), AppError> {
    assert!(max_hp_len > 0, "value of max_hp_len must be > 0.");

    let mut reader = needletail_fastx_reader(fasta)?;

    // Output file writer.
    let mut writer = get_bufwriter(outfile)?;

    while let Some(record) = reader.next() {
        let record = match record {
            Ok(record) => record,
            Err(_) => continue,
        };

        let compressed_sequence = homopolymer_compression(&record.seq(), max_hp_len);

        let id = std::str::from_utf8(record.id())?;

        // Id.
        writer.write_all(b">")?;
        writer.write_all(id.as_bytes())?;
        writer.write_all(b"\n")?;

        // Sequence
        writer.write_all(&compressed_sequence)?;
        writer.write_all(b"\n")?;
    }

    writer.flush()?;

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
