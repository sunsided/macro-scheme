//! Exact minimization. NP-hard in general: the branch-and-bound here is a
//! reference oracle for inputs of roughly 12 bytes or less. A MaxSAT
//! formulation (Bannai et al., ESA 2022) is the planned scalable solver.

use crate::encode::{lex_parse, lz77};
use crate::{MacroScheme, Phrase};

/// Returns a smallest valid macro scheme for `t`.
pub fn brute_force(t: &[u8]) -> MacroScheme {
    let a = lz77(t);
    let b = lex_parse(t);
    let mut best = if a.size() <= b.size() { a } else { b };
    let mut current = Vec::new();
    search(t, 0, &mut current, &mut best);
    best
}

fn search(t: &[u8], pos: usize, current: &mut Vec<Phrase>, best: &mut MacroScheme) {
    if pos == t.len() {
        let candidate = MacroScheme::new(current.clone());
        if candidate.size() < best.size() && candidate.validate().is_ok() {
            *best = candidate;
        }
        return;
    }
    if current.len() + 1 >= best.size() {
        return; // cannot beat the incumbent
    }
    // Longer copies first: they tend to find small schemes early.
    for len in (1..=t.len() - pos).rev() {
        let target = &t[pos..pos + len];
        for src in 0..=t.len() - len {
            if src != pos && &t[src..src + len] == target {
                current.push(Phrase::Copy { src, len });
                search(t, pos + len, current, best);
                current.pop();
            }
        }
    }
    current.push(Phrase::Literal(t[pos]));
    search(t, pos + 1, current, best);
    current.pop();
}
