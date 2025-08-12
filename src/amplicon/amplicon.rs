use crate::args::SearchType;
use crate::common::{AppError, bio_fasta_reader, get_bufwriter, reverse_complement, usize_sub};
use bio::pattern_matching::myers::MyersBuilder;
use memchr::memmem;
use rayon::prelude::*;
use rstest::*;
use std::collections::HashSet;
use std::path::PathBuf;

use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

#[derive(PartialEq, Debug)]
pub struct AmpliconResult<'a> {
    pub amplicon: &'a [u8],
    pub start: usize,
    pub end: usize,
    pub insert_length: usize,
    pub total_length: usize,
}
pub struct PrimerPair {
    pub primer_name: String,
    pub forward_primer: Vec<u8>,
    pub reverse_primer: Vec<u8>,
    pub min_len: usize,
    pub max_len: usize,
    pub num_mismatch: Option<usize>,
}

fn extract_primer_info(primer_line: &String) -> Result<PrimerPair, AppError> {
    let line_vec: Vec<&str> = primer_line.split("\t").map(|l| l.trim()).collect();

    if line_vec.len() < 5 {
        return Err(AppError::PrimerLineFormatError);
    }

    match line_vec.len() {
        5 => {
            // TODO - this is not optimal.
            let primer_name = line_vec[0];
            let forward_primer = line_vec[1].as_bytes();
            let reverse_primer = line_vec[2].as_bytes();
            let min_len = line_vec[3]
                .parse::<usize>()
                .map_err(|_| AppError::PrimerLenParsingError)?;
            let max_len = line_vec[4]
                .parse::<usize>()
                .map_err(|_| AppError::PrimerLenParsingError)?;

            // Try to extract number of mismatches from file if exists.
            let num_mismatch: Option<usize> = line_vec
                .get(5)
                .map(|s| {
                    s.parse::<usize>()
                        .map_err(|_| AppError::PrimerLineFormatError)
                })
                .transpose()?;

            Ok(PrimerPair {
                primer_name: primer_name.to_owned(),
                forward_primer: forward_primer.to_owned(),
                reverse_primer: reverse_primer.to_owned(),
                min_len: min_len,
                max_len: max_len,
                num_mismatch,
            })
        }
        _ => Err(AppError::PrimerLineFormatError),
    }
}

pub fn parse_primer_file<'a>(primer_file: &'a PathBuf) -> Result<Vec<PrimerPair>, AppError> {
    let f = File::open(primer_file).map_err(|_| AppError::PrimerFileParsingError)?;

    let reader = BufReader::new(f);

    let mut primer_pairs: Vec<PrimerPair> = Vec::new();

    for (_, line) in reader.lines().enumerate() {
        if let Ok(primer_line) = line {
            match extract_primer_info(&primer_line) {
                Ok(primer_pair) => primer_pairs.push(primer_pair),
                Err(_) => continue,
            }
        }
    }

    match primer_pairs.len() {
        0 => Err(AppError::NoPrimersFoundError),
        _ => Ok(primer_pairs),
    }
}

fn myers_builder(primer_seq: &[u8]) -> bio::pattern_matching::myers::Myers {
    return MyersBuilder::new()
        .ambig(b'N', b"ACGT")
        .ambig(b'R', b"AG")
        .ambig(b'Y', b"CT")
        .ambig(b'S', b"GC")
        .ambig(b'W', b"AT")
        .ambig(b'K', b"GT")
        .ambig(b'M', b"AC")
        .ambig(b'B', b"CGT")
        .ambig(b'D', b"AGT")
        .ambig(b'H', b"ACT")
        .ambig(b'V', b"ACG")
        .build_64(primer_seq);
}

#[allow(unused)]
pub fn amplicon_fuzzy_search<'a>(
    seq: &'a [u8],
    primer_pair: &PrimerPair,
) -> Vec<AmpliconResult<'a>> {
    let PrimerPair {
        primer_name,
        forward_primer,
        reverse_primer,
        min_len,
        max_len,
        num_mismatch,
    } = primer_pair;

    let num_mismatch = match num_mismatch {
        Some(num_mismatch) => *num_mismatch as u8,
        None => 1_u8,
    };

    let forward_len = forward_primer.len();
    let reverse_len: usize = reverse_primer.len();

    // For reverse primer, we need to 3' -> 5' direction.
    let reverse_complement_primer = reverse_complement(reverse_primer);
    let rcps = reverse_complement_primer.as_slice();

    let mut amplicons: Vec<AmpliconResult> = Vec::new();

    let mut myers_forward = myers_builder(&forward_primer);
    let mut myers_reverse = myers_builder(&rcps);

    // There is something about myers matching that causes duplicates, probably
    // multiple alignments in the same location. To prevent this, a temp solution
    // is to keep track of and avoid duplicate starting positions.
    //
    // We only allow unique forward primer starting positions.
    let mut forward_starts: HashSet<usize> = HashSet::new();

    for (forward_hit, _, _) in myers_forward.find_all(seq, num_mismatch) {
        let start = forward_hit + forward_len;

        if forward_starts.contains(&start) {
            continue;
        }
        forward_starts.insert(start);

        // For a given start primer match, we only allow unique reverse primer matching positions.
        let mut reverse_starts: HashSet<usize> = HashSet::new();
        for (reverse_hit, _, _) in myers_reverse.find_all(seq, num_mismatch) {
            let insert_length: usize = usize_sub(reverse_hit, start);

            if reverse_starts.contains(&reverse_hit) {
                continue;
            }

            reverse_starts.insert(reverse_hit);

            // If amplicon is within allowed length.
            if insert_length >= *min_len && insert_length <= *max_len {
                let amplicon = &seq[start..reverse_hit];

                let amplicon_result = AmpliconResult {
                    amplicon: amplicon,
                    start: start,
                    end: reverse_hit,
                    insert_length: insert_length,
                    total_length: forward_len + insert_length + reverse_len,
                };

                amplicons.push(amplicon_result);
            }
        }
    }

    return amplicons;
}
#[allow(unused)]
pub fn amplicon_exact_search<'a>(
    seq: &'a [u8],
    primer_pair: &PrimerPair,
) -> Vec<AmpliconResult<'a>> {
    let PrimerPair {
        primer_name,
        forward_primer,
        reverse_primer,
        min_len,
        max_len,
        num_mismatch,
    } = primer_pair;

    let forward_len = forward_primer.len();
    let reverse_len: usize = reverse_primer.len();

    // For reverse primer, we need to 3' -> 5' direction.
    let reverse_complement_primer = reverse_complement(reverse_primer);

    // Find all occurrences of the forward and reverse primers in seq.
    let forward_hits: Vec<usize> = memmem::find_iter(seq, forward_primer).collect();
    let reverse_hits: Vec<usize> =
        memmem::find_iter(seq, reverse_complement_primer.as_slice()).collect();

    let mut amplicons: Vec<AmpliconResult> = Vec::new();

    // This is not ideal if we expect many, many matches.
    for forward_hit in &forward_hits {
        let start = forward_hit + forward_len;

        for reverse_hit in &reverse_hits {
            let insert_length: usize = usize_sub(*reverse_hit, start);

            // If amplicon is within allowed length.
            if insert_length >= *min_len && insert_length <= *max_len {
                let amplicon = &seq[start..*reverse_hit];

                let amplicon_result = AmpliconResult {
                    amplicon: amplicon,
                    start: start,
                    end: *reverse_hit,
                    insert_length: insert_length,
                    total_length: forward_len + insert_length + reverse_len,
                };

                amplicons.push(amplicon_result);
            }
        }
    }

    return amplicons;
}

pub fn fasta_amplicon(
    fasta: Option<PathBuf>,
    primers: &PathBuf,
    search_type: &SearchType,
    outfile: Option<PathBuf>,
) -> Result<(), AppError> {
    // Read and parse primer file.
    let primer_pairs = parse_primer_file(primers)?;

    let reader = bio_fasta_reader(fasta)?;

    let mut writer = get_bufwriter(outfile).map_err(|_| AppError::FastaWriteError)?;

    writer
        .write_all(
            format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                "sequence_id",
                "primer_name",
                "start",
                "end",
                "insert_length",
                "actual_length",
                "amplicon"
            )
            .as_bytes(),
        )
        .map_err(|_| AppError::FastaWriteError)?;

    let search_function = match search_type {
        SearchType::Exact => amplicon_exact_search,
        SearchType::Fuzzy => amplicon_fuzzy_search,
    };

    let amplicon_results: Vec<Vec<String>> = reader
        .records()
        .par_bridge()
        .filter_map(|record| {
            if let Ok(record) = record {
                for primer_pair in &primer_pairs {
                    let amplicons = search_function(&record.seq(), primer_pair);

                    let result_vec: Vec<String> = amplicons
                        .iter()
                        .map(|amplicon| {
                            format!(
                                "{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                                record.id(),
                                primer_pair.primer_name,
                                amplicon.start,
                                amplicon.end,
                                amplicon.insert_length,
                                amplicon.total_length,
                                std::str::from_utf8(amplicon.amplicon).unwrap()
                            )
                        })
                        .collect();

                    return Some(result_vec);
                }
            }

            return None;
        })
        .collect();

    amplicon_results.iter().flatten().for_each(|r| {
        writer
            .write_all(r.as_bytes())
            .map_err(|_| AppError::FastaWriteError)
            .expect("Failed to write.");
    });

    Ok(())
}

#[rstest]
#[case(b"",
    &PrimerPair { forward_primer: b"A".to_vec(), reverse_primer: b"A".to_vec(), primer_name: "some_primer".to_string(), min_len: 0, max_len: 10, num_mismatch: Some(0)},
    vec![])]
#[case(b"ATCGTTTTTATCG",
    &PrimerPair { forward_primer: b"ATCG".to_vec(), reverse_primer: b"CGAT".to_vec(), primer_name: "some_primer".to_string(), min_len: 5, max_len: 5, num_mismatch: Some(0)},
    vec![AmpliconResult { amplicon: b"TTTTT", start: 4, end: 4 + 5, insert_length: 5, total_length: 4 + 5 + 4}])]
#[case(b"ATCGTTTTTATCGTTTTTATCG",
    &PrimerPair { forward_primer: b"ATCG".to_vec(), reverse_primer: b"CGAT".to_vec(), primer_name: "some_primer".to_string(), min_len: 5, max_len: 5, num_mismatch: Some(0)},
    vec![AmpliconResult { amplicon: b"TTTTT", start: 4, end: 4 + 5, insert_length: 5, total_length: 4 + 5 + 4},
         AmpliconResult { amplicon: b"TTTTT", start: 13, end: 13 + 5, insert_length: 5, total_length: 4 + 5 + 4}
    ])]

fn test_amplicon_exact_match(
    #[case] seq: &[u8],
    #[case] primer_pair: &PrimerPair,
    #[case] expected: Vec<AmpliconResult>,
) {
    assert_eq!(amplicon_exact_search(seq, primer_pair), expected)
}
