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
