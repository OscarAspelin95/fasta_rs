use rstest::*;
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

    (canonical, softmasked_count, ambiguous_count)
}

#[inline]
pub fn nucleotide_probabilities(canonical: &HashMap<&u8, usize>) -> Vec<f32> {
    let canonical_count = canonical.values().sum::<usize>();

    let probs: Vec<f32> = canonical
        .values()
        .map(|count| *count as f32 / canonical_count as f32)
        .collect();

    probs
}

#[inline]
pub fn shannon_entropy(probs: &[f32]) -> f32 {
    // Probabilities of each nucleotide.

    let shannon: f32 = probs
        .iter()
        .map(|prob| match prob {
            0_f32 => 0 as f32,
            // This is safe because prob is never negative since
            // both count and sum_count are of type usize.
            _ => prob * prob.log2(),
        })
        .sum();

    -shannon
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

    reverse_complement
}

#[inline]
pub fn usize_sub(a: usize, b: usize) -> usize {
    if a > b {
        return a - b;
    }

    0
}

#[rstest]
#[case(b"", 0, 0)]
#[case(b"A", 0, 0)]
#[case(b"AAaaTTtt", 4, 0)]
#[case(b"NNaattccggATCG", 8, 2)]
fn test_nucleotide_counts(
    #[case] seq: &[u8],
    #[case] expected_softmasked: usize,
    #[case] expected_ambiguous: usize,
) {
    let (_, num_softmasked, num_ambiguous) = nucleotide_counts(seq);

    assert_eq!(num_softmasked, expected_softmasked);
    assert_eq!(num_ambiguous, expected_ambiguous);
}

/// In these tests, we also check that not counting non-existing nucleotides
/// is the same as setting non-existing nucleotide probabilities to 0.0.
#[rstest]
#[case(vec![], 0.0)]
#[case(vec![0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32], 0.0)]
#[case(vec![1.0_f32], 0.0)]
#[case(vec![1.0_f32, 0.0_f32, 0.0_f32, 0.0_f32], 0.0)]
#[case(vec![0.5_f32, 0.5_f32], 1.0)]
#[case(vec![0.5_f32, 0.5_f32, 0.0_f32, 0.0_f32], 1.0)]
fn test_shannon_entropy(#[case] probs: Vec<f32>, #[case] expected: f32) {
    assert_eq!(shannon_entropy(&probs), expected);
}

#[rstest]
#[case(b"", 0.0_f32)]
#[case(b"ATCG", 0.5_f32)]
#[case(b"ATAT", 0.0_f32)]
#[case(b"CGCG", 1.0_f32)]
#[case(b"ATCT", 0.25_f32)]
fn test_gc_content(#[case] seq: &[u8], #[case] expected: f32) {
    assert_eq!(gc_content(seq), expected);
}

#[rstest]
#[case(b"", b"")]
#[case(b"A", b"T")]
#[case(b"C", b"G")]
#[case(b"G", b"C")]
#[case(b"T", b"A")]
#[case(b"AAAAA", b"TTTTT")]
#[case(b"ATCG", b"CGAT")]
fn test_reverse_complement(#[case] seq: &[u8], #[case] expected: &[u8]) {
    assert_eq!(reverse_complement(seq).as_slice(), expected);
}

#[rstest]
#[case(0, 0, 0)]
#[case(5, 2, 3)]
#[case(5, 10, 0)]
fn test_usize_sub(#[case] a: usize, #[case] b: usize, #[case] expected: usize) {
    assert_eq!(usize_sub(a, b), expected);
}
