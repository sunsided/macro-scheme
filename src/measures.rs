//! Repetitiveness measures used as bounds on the optimal scheme size `b`.

use std::collections::HashSet;

/// Substring complexity `δ = max_k d_k / k`, where `d_k` is the number of
/// distinct length-`k` substrings. A lower bound on `b`. Naive `O(n²)`.
pub fn delta(t: &[u8]) -> f64 {
    (1..=t.len())
        .map(|k| t.windows(k).collect::<HashSet<_>>().len() as f64 / k as f64)
        .fold(0.0, f64::max)
}

/// Number of LZ77 phrases: an upper bound on `b`.
pub fn z(t: &[u8]) -> usize {
    crate::encode::lz77(t).size()
}
