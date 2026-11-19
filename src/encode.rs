//! Encoders producing valid macro schemes.

use crate::suffix::{lcp, suffix_array};
use crate::{MacroScheme, Phrase};

/// Greedy LZ77 with self-overlapping sources: every copy points strictly
/// left, so the scheme is trivially valid. Its size is `z`.
/// Naive quadratic search; fine for tests, replace with an SA-based
/// factorizer for large inputs.
#[must_use]
pub fn lz77(t: &[u8]) -> MacroScheme {
    let mut phrases = Vec::new();
    let mut i = 0;

    while i < t.len() {
        let (src, len) = longest_previous_match(t, i);
        let phrase = phrase_at(t, i, src, len);

        i += phrase.len();
        phrases.push(phrase);
    }

    MacroScheme::new(phrases)
}

/// Lex-parse (Navarro, Ochoa & Prezza, 2021): the phrase at `i` copies from
/// the lexicographically preceding suffix, for as long as they agree. Sources
/// may lie to the right. Every pointer goes to a lexicographically smaller
/// suffix, so no cycle is possible. Size is at most `2r`.
#[must_use]
pub fn lex_parse(t: &[u8]) -> MacroScheme {
    let sa = suffix_array(t);
    let mut rank = vec![0; t.len()];
    for (r, &s) in sa.iter().enumerate() {
        rank[s] = r;
    }

    let mut phrases = Vec::new();
    let mut i = 0;

    while i < t.len() {
        let (src, len) = match rank[i] {
            0 => (0, 0),
            r => (sa[r - 1], lcp(t, i, sa[r - 1])),
        };
        let phrase = phrase_at(t, i, src, len);

        i += phrase.len();
        phrases.push(phrase);
    }

    MacroScheme::new(phrases)
}

/// The leftmost longest match for the suffix at `i` that starts strictly before `i`,
/// as `(src, len)`. `len` is `0` when no earlier position shares the first byte.
fn longest_previous_match(t: &[u8], i: usize) -> (usize, usize) {
    let (mut best_src, mut best_len) = (0, 0);

    for src in 0..i {
        let l = lcp(t, src, i);
        if l > best_len {
            (best_src, best_len) = (src, l);
        }
    }

    (best_src, best_len)
}

/// The phrase starting at text position `i`: a copy of `len` bytes from `src`, or the
/// literal `t[i]` when the match is empty.
fn phrase_at(t: &[u8], i: usize, src: usize, len: usize) -> Phrase {
    if len == 0 {
        Phrase::Literal(t[i])
    } else {
        Phrase::Copy { src, len }
    }
}
