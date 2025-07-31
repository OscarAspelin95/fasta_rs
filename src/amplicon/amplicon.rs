use crate::common::{AppError, bio_fasta_reader, reverse_complement, usize_sub};
use log::warn;
use memchr::memmem;
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

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
}

fn extract_primer_info(primer_line: &String) -> Result<PrimerPair, AppError> {
    let line_vec: Vec<&str> = primer_line.split("\t").map(|l| l.trim()).collect();

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

            Ok(PrimerPair {
                primer_name: primer_name.to_owned(),
                forward_primer: forward_primer.to_owned(),
                reverse_primer: reverse_primer.to_owned(),
                min_len: min_len,
                max_len: max_len,
            })
        }
        _ => Err(AppError::PrimerLineFormatError),
    }
}

pub fn parse_primer_file<'a>(primer_file: &'a PathBuf) -> Result<Vec<PrimerPair>, AppError> {
    let f = File::open(primer_file).map_err(|_| AppError::PrimerFileParsingError)?;

    let reader = BufReader::new(f);

    let mut primer_pairs: Vec<PrimerPair> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        if let Ok(primer_line) = line {
            match extract_primer_info(&primer_line) {
                Ok(primer_pair) => primer_pairs.push(primer_pair),
                Err(e) => warn!("{}, line: {} `{}`", e, i, primer_line),
            }
        }
    }

    match primer_pairs.len() {
        0 => Err(AppError::NoPrimersFoundError),
        _ => Ok(primer_pairs),
    }
}
fn write_header(writer: &Arc<Mutex<BufWriter<File>>>) {
    writer
        .lock()
        .unwrap()
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
        .unwrap();
}

#[allow(unused)]
pub fn amplicon_search<'a>(seq: &'a [u8], primer_pair: &PrimerPair) -> Vec<AmpliconResult<'a>> {
    let PrimerPair {
        primer_name,
        forward_primer,
        reverse_primer,
        min_len,
        max_len,
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
    fasta: &PathBuf,
    primers: &PathBuf,
    outfile: &PathBuf,
) -> Result<(), AppError> {
    // Read and parse primer file.
    let primer_pairs = parse_primer_file(primers)?;

    let reader = bio_fasta_reader(fasta)?;

    // Initialize writer to which we write results.
    let writer = Arc::new(Mutex::new(BufWriter::new(
        File::create(outfile).expect("Failed to create output file."),
    )));

    // Write tsv header.
    write_header(&writer);

    reader.records().par_bridge().for_each(|record| {
        if let Ok(record) = record {
            for primer_pair in &primer_pairs {
                let amplicons = amplicon_search(record.seq(), primer_pair);

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

                // Is is probably not ideal to write results after each primer pair.
                // A better approach would be to write after each record.
                if result_vec.len() > 0 {
                    writer
                        .lock()
                        .unwrap()
                        .write_all(result_vec.concat().as_bytes())
                        .unwrap();
                }
            }
        }
    });

    Ok(())
}
