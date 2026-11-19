//! Exact minimization. NP-hard in general: the branch-and-bound here is a
//! reference oracle for inputs of roughly 12 bytes or less. A `MaxSAT`
//! formulation (Bannai et al., ESA 2022) is the planned scalable solver.

use crate::encode::{lex_parse, lz77};
use crate::{MacroScheme, Phrase};

/// Returns a smallest valid macro scheme for `t`.
///
/// Exponential in `t.len()`; starts from the smaller of [`lz77`] and [`lex_parse`].
#[must_use]
pub fn brute_force(t: &[u8]) -> MacroScheme {
    let a = lz77(t);
    let b = lex_parse(t);
    let mut best = if a.size() <= b.size() { a } else { b };

    search(t, 0, &mut Vec::new(), &mut best);
    best
}

/// Depth-first branch and bound over every parsing of `t[pos..]`.
///
/// `current` holds the phrases chosen for `t[..pos]`; `best` is the smallest valid
/// scheme found so far and is replaced whenever a smaller valid one completes.
fn search(t: &[u8], pos: usize, current: &mut Vec<Phrase>, best: &mut MacroScheme) {
    if pos == t.len() {
        keep_if_better(current, best);
        return;
    }

    if current.len() + 1 >= best.size() {
        return; // cannot beat the incumbent
    }

    // Longer copies first: they tend to find small schemes early.
    for len in (1..=t.len() - pos).rev() {
        for src in copy_sources(t, pos, len) {
            branch(t, Phrase::Copy { src, len }, pos + len, current, best);
        }
    }

    branch(t, Phrase::Literal(t[pos]), pos + 1, current, best);
}

/// Tries `phrase` as the next phrase, searches the rest from `next_pos`, then undoes it.
fn branch(
    t: &[u8],
    phrase: Phrase,
    next_pos: usize,
    current: &mut Vec<Phrase>,
    best: &mut MacroScheme,
) {
    current.push(phrase);
    search(t, next_pos, current, best);
    current.pop();
}

/// Every source position other than `pos` whose `len` bytes equal `t[pos..pos + len]`.
fn copy_sources(t: &[u8], pos: usize, len: usize) -> impl Iterator<Item = usize> + '_ {
    let target = &t[pos..pos + len];

    (0..=t.len() - len).filter(move |&src| src != pos && &t[src..src + len] == target)
}

/// Replaces `best` with the complete parsing in `current` if it is smaller and valid.
fn keep_if_better(current: &[Phrase], best: &mut MacroScheme) {
    let candidate = MacroScheme::new(current.to_vec());
    if candidate.size() < best.size() && candidate.validate().is_ok() {
        *best = candidate;
    }
}
