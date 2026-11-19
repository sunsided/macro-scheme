//! Encoders producing valid macro schemes.

use crate::suffix::{lcp, suffix_array};
use crate::{MacroScheme, Phrase};

/// Greedy LZ77 with self-overlapping sources: every copy points strictly
/// left, so the scheme is trivially valid. Its size is `z`.
/// Naive quadratic search; fine for tests, replace with an SA-based
/// factorizer for large inputs.
pub fn lz77(t: &[u8]) -> MacroScheme {
    let mut phrases = Vec::new();
    let mut i = 0;
    while i < t.len() {
        let (mut best_src, mut best_len) = (0, 0);
        for src in 0..i {
            let l = lcp(t, src, i);
            if l > best_len {
                (best_src, best_len) = (src, l);
            }
        }
        if best_len == 0 {
            phrases.push(Phrase::Literal(t[i]));
            i += 1;
        } else {
            phrases.push(Phrase::Copy {
                src: best_src,
                len: best_len,
            });
            i += best_len;
        }
    }
    MacroScheme::new(phrases)
}

/// Lex-parse (Navarro, Ochoa & Prezza, 2021): the phrase at `i` copies from
/// the lexicographically preceding suffix, for as long as they agree. Sources
/// may lie to the right. Every pointer goes to a lexicographically smaller
/// suffix, so no cycle is possible. Size is at most `2r`.
pub fn lex_parse(t: &[u8]) -> MacroScheme {
    let sa = suffix_array(t);
    let mut rank = vec![0; t.len()];
    for (r, &s) in sa.iter().enumerate() {
        rank[s] = r;
    }
    let mut phrases = Vec::new();
    let mut i = 0;
    while i < t.len() {
        let l = if rank[i] == 0 {
            0
        } else {
            lcp(t, i, sa[rank[i] - 1])
        };
        if l == 0 {
            phrases.push(Phrase::Literal(t[i]));
            i += 1;
        } else {
            phrases.push(Phrase::Copy {
                src: sa[rank[i] - 1],
                len: l,
            });
            i += l;
        }
    }
    MacroScheme::new(phrases)
}
