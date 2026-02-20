use crate::errors::AppError;
use bio_utils_rs::io::{get_bufwriter, needletail_reader};
use bio_utils_rs::nucleotide::{nucleotide_counts, nucleotide_probabilities, shannon_entropy};
use std::path::PathBuf;

#[allow(unused)]
#[allow(clippy::too_many_arguments)]
pub fn fasta_filter(
    fasta: Option<PathBuf>,
    min_len: usize,
    max_len: usize,
    min_gc: f32,
    max_gc: f32,
    min_ambig: f32,
    max_ambig: f32,
    min_softmask: f32,
    max_softmask: f32,
    min_entropy: f32,
    max_entropy: f32,
    outfile: Option<PathBuf>,
) -> Result<(), AppError> {
    let mut reader = needletail_reader(fasta)?;
    let mut writer = get_bufwriter(outfile)?;

    while let Some(record) = reader.next() {
        let record = match record {
            Ok(record) => record,
            Err(_) => continue,
        };

        let num_bases = record.seq().len();

        // Handle length criteria early.
        if num_bases < min_len || num_bases > max_len {
            continue;
        }

        let record_seq = record.seq();

        // Nucleotide counts.
        let (canonical, softmask_count, ambiguous_count) = nucleotide_counts(&record_seq);

        // Softmask.
        let softmask_fraction = softmask_count as f32 / num_bases as f32;
        if softmask_fraction < min_softmask || softmask_fraction > max_softmask {
            continue;
        }

        // Ambiguous.
        let ambiguous_fraction = ambiguous_count as f32 / num_bases as f32;
        if ambiguous_fraction < min_ambig || ambiguous_fraction > max_ambig {
            continue;
        }

        // GC.
        let canonical_count: usize = canonical.iter().sum();

        let g_count: usize = canonical[2]; // G
        let c_count: usize = canonical[1]; // C

        let gc_fraction: f32 = (g_count as f32 + c_count as f32) / canonical_count as f32;
        if gc_fraction < min_gc || gc_fraction > max_gc {
            continue;
        }

        // Entropy
        let probs: Vec<f32> = nucleotide_probabilities(&canonical);
        let entropy = shannon_entropy(&probs);

        if entropy < min_entropy || entropy > max_entropy {
            continue;
        }

        record.write(&mut writer, None)?;
    }

    writer.flush()?;

    Ok(())
}
