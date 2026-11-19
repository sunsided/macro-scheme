//! Fuzzes [`MacroScheme::decode`] with arbitrary, mostly invalid phrase lists.
//!
//! Decoding must either reject the scheme with an [`InvalidScheme`] error or return a text
//! whose length matches [`MacroScheme::text_len`]. It must never panic.

#![no_main]

use libfuzzer_sys::fuzz_target;
use macro_scheme::{InvalidScheme, MacroScheme, Phrase};

/// Largest text a valid scheme may describe before it is skipped. A valid scheme for a
/// huge text must allocate that text, which is not a bug but exhausts the fuzzer's memory.
const MAX_VALID_TEXT_LEN: usize = 1 << 20;

/// Returns `true` when the scheme could be valid and describe a text longer than
/// [`MAX_VALID_TEXT_LEN`]: every copy then lies inside the text.
fn is_huge_and_possibly_valid(scheme: &MacroScheme) -> bool {
    let Some(n) = scheme
        .phrases()
        .iter()
        .try_fold(0usize, |n, p| n.checked_add(p.len()))
    else {
        return false;
    };

    n > MAX_VALID_TEXT_LEN
        && scheme.phrases().iter().all(|p| match *p {
            Phrase::Literal(_) => true,
            Phrase::Copy { src, len } => {
                len > 0 && src.checked_add(len).is_some_and(|end| end <= n)
            }
        })
}

fuzz_target!(|scheme: MacroScheme| {
    if is_huge_and_possibly_valid(&scheme) {
        return;
    }

    let result: Result<Vec<u8>, InvalidScheme> = scheme.decode();

    if let Ok(text) = result {
        assert_eq!(text.len(), scheme.text_len());
    }
});
