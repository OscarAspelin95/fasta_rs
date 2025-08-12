use crate::common::{AppError, get_bufwriter, needletail_fastx_reader};
use log::{info, warn};
use needletail::parser::SequenceRecord;
use rstest::*;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

#[inline]
fn u8_to_char(nt: &u8) -> Option<char> {
    match nt {
        // Default nucleotides.
        b'A' => return Some('A'),
        b'T' => return Some('T'),
        b'C' => return Some('C'),
        b'G' => return Some('G'),
        // Soft masked nucleotides.
        b'a' => return Some('a'),
        b't' => return Some('t'),
        b'c' => return Some('c'),
        b'g' => return Some('g'),
        _ => return None,
    }
}

#[inline]
fn valid_homopolymer(i: usize, j: usize, nt: &u8, min_hp_len: usize, strict: bool) -> bool {
    let valid_len = j - i >= min_hp_len;

    match (strict, u8_to_char(&nt)) {
        (false, _) => return valid_len,
        (true, Some(_)) => return valid_len,
        (true, None) => return false,
    }
}
#[inline]
pub fn find_homopolymers_in_record(
    record: &SequenceRecord,
    min_hp_len: usize,
    strict: bool,
    bufwriter: &mut BufWriter<File>,
) {
    // Extract sequence information.
    let seq_name = record.id();
    let seq = record.seq();
    let seq_len = seq.len();

    // Skip sequence if shorter than hp len.
    if seq_len < min_hp_len {
        warn!(
            "Skipping record {} (too short)",
            std::str::from_utf8(seq_name).unwrap()
        );
        return;
    }

    let mut i = 0;
    let mut j = 1;

    while i <= seq_len - min_hp_len {
        while j < seq_len && seq[j] == seq[i] {
            j += 1;
        }

        // We have a homopolymer of required length.
        if valid_homopolymer(i, j, &seq[i], min_hp_len, strict) {
            let s = format!(
                "{}\t{}\t{}\t{}\t{}\n",
                std::str::from_utf8(seq_name).unwrap(),
                i,
                j,
                j - i,
                char::from(seq[i])
            );

            bufwriter.write_all(s.as_bytes()).unwrap();
        }

        i = j;
        j += 1;
    }
}

pub fn fasta_homopolymers(
    fasta: Option<PathBuf>,
    min_hp_len: usize,
    strict: bool,
    outfile: &PathBuf,
) -> Result<(), AppError> {
    let mut reader = needletail_fastx_reader(fasta)?;

    // Output file writer.
    let mut bufwriter = get_bufwriter(outfile)?;

    // Write tsv header.
    let s = format!(
        "{}\t{}\t{}\t{}\t{}\n",
        "contig", "start", "end", "len", "nt"
    );
    bufwriter.write_all(s.as_bytes()).unwrap();

    info!("Finding homopolymers...");
    while let Some(record) = reader.next() {
        match record {
            Ok(record) => find_homopolymers_in_record(&record, min_hp_len, strict, &mut bufwriter),
            Err(e) => {
                warn!("Failed to parse record: {:?}", e);
            }
        }
    }
    Ok(())
}

#[rstest]
#[case(0, 5, &b'A', 5, false, true)]
#[case(0, 1, &b'A', 5, false, false)]
#[case(0, 10, &b'N', 5, true, false)]

fn test_valid_homopolymer(
    #[case] i: usize,
    #[case] j: usize,
    #[case] nt: &u8,
    #[case] min_hp_len: usize,
    #[case] strict: bool,
    #[case] expected_valid: bool,
) {
    assert_eq!(
        valid_homopolymer(i, j, nt, min_hp_len, strict),
        expected_valid
    );
}
