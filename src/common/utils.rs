use std::collections::HashMap;

#[inline]
pub fn nucleotide_counts(seq: &[u8]) -> (HashMap<&u8, usize>, usize, usize) {
    let mut canonical: HashMap<&u8, usize> = HashMap::with_capacity(4);

    // Counts of non-canonical nucleotides.
    let mut softmasked_count: usize = 0;
    let mut ambiguous_count: usize = 0;

    for nt in seq.iter() {
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

    return (canonical, softmasked_count, ambiguous_count);
}

#[inline]
pub fn nucleotide_probabilities(canonical: &HashMap<&u8, usize>) -> Vec<f32> {
    let canonical_count = canonical.values().sum::<usize>();

    let probs: Vec<f32> = canonical
        .values()
        .map(|count| *count as f32 / canonical_count as f32)
        .collect();

    return probs;
}

#[inline]
pub fn shannon_entropy(probs: &Vec<f32>) -> f32 {
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
#[inline]
pub fn gc_content(seq: &[u8]) -> f32 {
    let mut num_bases: usize = 0;
    let mut gc_count: usize = 0;

    for nt in seq {
        num_bases += 1;

        if nt == &b'G' || nt == &b'C' || nt == &b'g' || nt == &b'c' {
            gc_count += 1;
        };
    }

    match gc_count {
        0 => 0.0,
        _ => gc_count as f32 / num_bases as f32,
    }
}

#[inline]
pub fn reverse_complement(seq: &[u8]) -> Vec<u8> {
    let reverse_complement: Vec<u8> = seq
        .iter()
        .rev()
        .map(|nt| match nt {
            b'A' => b'T',
            b'C' => b'G',
            b'G' => b'C',
            b'T' => b'A',
            _ => panic!(""),
        })
        .collect();

    return reverse_complement;
}

#[inline]
pub fn usize_sub(a: usize, b: usize) -> usize {
    if a > b {
        return a - b;
    }

    return 0;
}
