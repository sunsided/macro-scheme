//! Suffix array and LCP. Naive `O(n² log n)` construction for now; swap in
//! SA-IS (e.g. the `libsais` crate) before benchmarking.

pub(crate) fn suffix_array(t: &[u8]) -> Vec<usize> {
    let mut sa: Vec<usize> = (0..t.len()).collect();
    sa.sort_unstable_by(|&a, &b| t[a..].cmp(&t[b..]));
    sa
}

pub(crate) fn lcp(t: &[u8], a: usize, b: usize) -> usize {
    t[a..]
        .iter()
        .zip(&t[b..])
        .take_while(|(x, y)| x == y)
        .count()
}
