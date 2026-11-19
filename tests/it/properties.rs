//! Property tests: encoder round-trips, the measure hierarchy, and decoding robustness.

use crate::as_f64;
use crate::strategies::{any_scheme, small_alphabet};
use macro_scheme::encode::{lex_parse, lz77};
use macro_scheme::{measures, optimal};
use proptest::prelude::*;

proptest! {
    /// Both encoders produce valid schemes that decode back to the input.
    #[test]
    fn encoders_round_trip(t in small_alphabet(200)) {
        prop_assert_eq!(lz77(&t).decode().unwrap(), t.clone());
        prop_assert_eq!(lex_parse(&t).decode().unwrap(), t);
    }

    /// δ ≤ b ≤ min(z, lex-parse) on every input.
    #[test]
    fn optimum_respects_the_measure_hierarchy(t in small_alphabet(9)) {
        let b = optimal::brute_force(&t);

        prop_assert_eq!(b.decode().unwrap(), t.clone());
        prop_assert!(as_f64(b.size()) >= measures::delta(&t) - 1e-12);
        prop_assert!(b.size() <= measures::z(&t));
        prop_assert!(b.size() <= lex_parse(&t).size());
    }

    /// Decoding any scheme returns an error or a text of `text_len` bytes, and never panics.
    #[test]
    fn decode_never_panics(scheme in any_scheme()) {
        if let Ok(text) = scheme.decode() {
            prop_assert_eq!(text.len(), scheme.text_len());
            prop_assert!(scheme.validate().is_ok());
        }
    }
}
