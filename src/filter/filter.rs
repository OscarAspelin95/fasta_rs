use crate::common::{AppError, needletail_fasta_reader};
use std::collections::HashMap;
use std::path::PathBuf;

#[inline]
fn shannon_entropy(probs: &Vec<f32>) -> f32 {
    // Probabilities of each nucleotide.

    let shannon: f32 = probs
        .iter()
        .map(|prob| match prob {
            0_f32 => return 0 as f32,
            // This is safe because prob is never negative since
            // both count and sum_count are of type usize.
            _ => {
                return prob * prob.log2();
            }
        })
        .sum();

    return -shannon;
}
#[allow(unused)]
pub fn fasta_filter(
    fasta: &PathBuf,
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
    outfile: &PathBuf,
) -> Result<(), AppError> {
    let mut reader = needletail_fasta_reader(fasta)?;

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

        // Store canonical NTs, which is used for GC content and entropy.
        let mut canonical: HashMap<&u8, usize> = HashMap::with_capacity(4);

        // Counts of non-canonical nucleotides.
        let mut softmasked_count: usize = 0;
        let mut ambiguous_count: usize = 0;

        let record_seq = record.seq();

        for nt in record_seq.iter() {
            match nt {
                // Canonical.
                b'A' | b'C' | b'G' | b'T' => {
                    canonical
                        .entry(nt)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }

                // Softmasked.
                b'a' | b'c' | b'g' | b't' => {
                    softmasked_count += 1;
                }

                // Ambiguous
                _ => {
                    ambiguous_count += 1;
                }
            }
        }

        // Softmask.
        let softmask_fraction = softmasked_count as f32 / num_bases as f32;
        if softmask_fraction < min_softmask || softmask_fraction > max_softmask {
            continue;
        }

        // Ambiguous.
        let ambiguous_fraction = ambiguous_count as f32 / num_bases as f32;
        if ambiguous_fraction < min_ambig || ambiguous_fraction > max_ambig {
            continue;
        }

        // GC. NOTE - should we include softmasked bases in gc count?
        let canonical_count = canonical.values().sum::<usize>();

        let g_count: usize = *canonical.get(&b'G').unwrap_or(&0);
        let c_count: usize = *canonical.get(&b'C').unwrap_or(&0);

        let gc_fraction: f32 = (g_count as f32 + c_count as f32) / canonical_count as f32;
        if gc_fraction < min_gc || gc_fraction > max_gc {
            continue;
        }

        // Entropy
        let probs: Vec<f32> = canonical
            .values()
            .map(|count| *count as f32 / canonical_count as f32)
            .collect();

        let entropy = shannon_entropy(&probs);

        if entropy < min_entropy || entropy > max_entropy {
            continue;
        }

        println!(
            ">{}\n{}",
            std::str::from_utf8(record.id()).unwrap(),
            std::str::from_utf8(&record_seq).unwrap()
        );
    }

    Ok(())
}
