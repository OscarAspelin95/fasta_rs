use crate::args::SortType;
use crate::common::get_bufwriter;
use crate::common::utils::nucleotide_probabilities;
use crate::common::writer::get_fasta_writer;
use crate::common::{AppError, bio_fasta_reader, gc_content, nucleotide_counts, shannon_entropy};
use bio::io::fasta::Record;
use log::info;
use rayon::prelude::*;
use serde::Serialize;
use std::cmp::Ordering;
use std::path::PathBuf;
use std::usize;

#[derive(Debug, Serialize)]
struct FastaRecord {
    record: Record,
    gc: f32,
    entropy: f32,
    softmask_count: usize,
    ambiguous_count: usize,
}

fn ascending_or_descending<T: PartialOrd>(ordering: Ordering, reverse: bool) -> Ordering {
    match reverse {
        false => ordering,
        true => ordering.reverse(),
    }
}

pub fn fasta_sort(
    fasta: &PathBuf,
    sort_type: SortType,
    reverse: bool,
    outfile: &PathBuf,
) -> Result<(), AppError> {
    let reader = bio_fasta_reader(fasta)?;

    info!("Parsing records...");
    let mut fasta_records: Vec<FastaRecord> = reader
        .records()
        .par_bridge()
        .filter_map(|record| {
            let record = record.ok()?;

            let record_seq = record.seq();

            // GC count
            let gc = gc_content(record_seq);

            // Shannon Entropy
            let (canonical, softmask_count, ambiguous_count) = nucleotide_counts(&record_seq);
            let probs = nucleotide_probabilities(&canonical);
            let entropy = shannon_entropy(&probs);

            let fasta_record = FastaRecord {
                record: record,
                gc: gc,
                entropy: entropy,
                softmask_count: softmask_count,
                ambiguous_count: ambiguous_count,
            };

            return Some(fasta_record);
        })
        .collect();

    // TODO - move this to separate function.
    // Sorting requires cmp or partial_cmp depending on data type.
    info!("Sorting records...");
    match sort_type {
        SortType::Length => {
            fasta_records.par_sort_by(|a, b| {
                let ord = a.record.seq().len().cmp(&b.record.seq().len());
                ascending_or_descending::<usize>(ord, reverse)
            });
        }
        SortType::Id => {
            fasta_records.par_sort_by(|a: &FastaRecord, b| {
                let ord = a.record.id().cmp(&b.record.id());
                ascending_or_descending::<&str>(ord, reverse)
            });
        }
        SortType::Gc => {
            fasta_records.par_sort_by(|a, b| {
                let ord = a.gc.partial_cmp(&b.gc).unwrap();
                ascending_or_descending::<f32>(ord, reverse)
            });
        }
        SortType::Entropy => {
            fasta_records.par_sort_by(|a, b| {
                let ord = a
                    .entropy
                    .partial_cmp(&b.entropy)
                    .expect("Invalid GC content, cannot sort.");

                ascending_or_descending::<f32>(ord, reverse)
            });
        }
        SortType::Softmask => {
            fasta_records.par_sort_by(|a: &FastaRecord, b| {
                let ord: Ordering = a.softmask_count.cmp(&b.softmask_count);
                ascending_or_descending::<usize>(ord, reverse)
            });
        }
        SortType::Ambiguous => {
            fasta_records.par_sort_by(|a: &FastaRecord, b| {
                let ord = a.ambiguous_count.cmp(&b.ambiguous_count);
                ascending_or_descending::<usize>(ord, reverse)
            });
        }
    }

    let mut writer = get_fasta_writer(outfile)?;

    for fasta_record in fasta_records {
        writer
            .write_record(&fasta_record.record)
            .map_err(|_| AppError::FastaWriteError)?;
    }

    Ok(())
}
