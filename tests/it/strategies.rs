//! proptest strategies shared by the property tests.

use macro_scheme::{MacroScheme, Phrase};
use proptest::prelude::*;

/// Non-empty texts of at most `max_len` bytes over the alphabet `{a, b, c}`.
///
/// A small alphabet makes repeats, and therefore copies, likely.
pub fn small_alphabet(max_len: usize) -> impl Strategy<Value = Vec<u8>> {
    proptest::collection::vec(prop_oneof![Just(b'a'), Just(b'b'), Just(b'c')], 1..=max_len)
}

/// Arbitrary, mostly invalid schemes of up to 12 phrases.
///
/// Sources and lengths are small enough that some schemes are valid, and large enough that
/// many copies are empty, out of bounds or cyclic.
pub fn any_scheme() -> impl Strategy<Value = MacroScheme> {
    let phrase = prop_oneof![
        any::<u8>().prop_map(Phrase::Literal),
        (0..40usize, 0..10usize).prop_map(|(src, len)| Phrase::Copy { src, len }),
    ];

    proptest::collection::vec(phrase, 0..12).prop_map(MacroScheme::new)
}
