use crate::errors::AppError;
use bio_utils_rs::io::{get_bufwriter, needletail_reader};
use needletail::parser::SequenceRecord;
use rstest::*;
use std::io::Write;
use std::path::PathBuf;

#[inline]
fn u8_to_char(nt: &u8) -> Option<char> {
    match nt {
        // Default nucleotides.
        b'A' => Some('A'),
        b'T' => Some('T'),
        b'C' => Some('C'),
        b'G' => Some('G'),
        // Soft masked nucleotides.
        b'a' => Some('a'),
        b't' => Some('t'),
        b'c' => Some('c'),
        b'g' => Some('g'),
        _ => None,
    }
}

#[inline]
fn valid_homopolymer(i: usize, j: usize, nt: &u8, min_hp_len: usize, strict: bool) -> bool {
    let valid_len = j - i >= min_hp_len;

    match (strict, u8_to_char(nt)) {
        (false, _) => valid_len,
        (true, Some(_)) => valid_len,
        (true, None) => false,
    }
}
#[inline]
pub fn find_homopolymers_in_record(
    record: &SequenceRecord,
    min_hp_len: usize,
    strict: bool,
    writer: &mut Box<dyn Write + Send>,
) -> Result<(), AppError> {
    // Extract sequence information.
    let seq = record.seq();
    let seq_len = seq.len();

    // Skip sequence if shorter than hp len.
    if seq_len < min_hp_len {
        return Ok(());
    }

    let mut i = 0;
    let mut j = 1;

    while i <= seq_len - min_hp_len {
        while j < seq_len && seq[j] == seq[i] {
            j += 1;
        }

        // We have a homopolymer of required length.
        if valid_homopolymer(i, j, &seq[i], min_hp_len, strict) {
            // Id.
            writer.write_all(record.id())?;
            writer.write_all(b"\t")?;
            // Start of hp.
            writer.write_all(i.to_string().as_bytes())?;
            writer.write_all(b"\t")?;

            // End of hp.
            writer.write_all(j.to_string().as_bytes())?;
            writer.write_all(b"\t")?;

            // Hp length.
            writer.write_all((j - i).to_string().as_bytes())?;
            writer.write_all(b"\t")?;

            // Hp nucleotide.
            writer.write_all(&[seq[i]])?;
            writer.write_all(b"\n")?;
        }

        i = j;
        j += 1;
    }

    writer.flush()?;

    Ok(())
}

pub fn fasta_homopolymers(
    fasta: Option<PathBuf>,
    min_hp_len: usize,
    strict: bool,
    outfile: Option<PathBuf>,
) -> Result<(), AppError> {
    let mut reader = needletail_reader(fasta)?;

    // Output file writer.
    let mut writer = get_bufwriter(outfile)?;

    // Write tsv header.
    let s = format!(
        "{}\t{}\t{}\t{}\t{}\n",
        "contig", "start", "end", "len", "nt"
    );
    writer.write_all(s.as_bytes()).unwrap();

    while let Some(record) = reader.next() {
        let record = match record {
            Ok(record) => record,
            Err(_) => continue,
        };

        find_homopolymers_in_record(&record, min_hp_len, strict, &mut writer)?
    }

    writer.flush()?;

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
