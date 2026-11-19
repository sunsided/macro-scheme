use macro_scheme::encode::{lex_parse, lz77};
use macro_scheme::{InvalidScheme, MacroScheme, Phrase, measures, optimal};
use proptest::prelude::*;

fn small_alphabet(max_len: usize) -> impl Strategy<Value = Vec<u8>> {
    proptest::collection::vec(prop_oneof![Just(b'a'), Just(b'b'), Just(b'c')], 1..=max_len)
}

proptest! {
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
        prop_assert!(b.size() as f64 >= measures::delta(&t) - 1e-12);
        prop_assert!(b.size() <= measures::z(&t));
        prop_assert!(b.size() <= lex_parse(&t).size());
    }
}

#[test]
fn rejects_cycles_and_bad_sources() {
    // "ab" where each position copies the other: no literal is ever reached.
    let cyclic = MacroScheme::new(vec![
        Phrase::Copy { src: 1, len: 1 },
        Phrase::Copy { src: 0, len: 1 },
    ]);
    assert!(matches!(
        cyclic.validate(),
        Err(InvalidScheme::Cycle { .. })
    ));
    let out = MacroScheme::new(vec![Phrase::Literal(b'a'), Phrase::Copy { src: 2, len: 5 }]); // ends at 7 > 6
    assert_eq!(
        out.validate(),
        Err(InvalidScheme::SourceOutOfBounds { phrase: 1 })
    );
}

#[test]
fn fibonacci_word_fixture() {
    let mut fib = (b"a".to_vec(), b"ab".to_vec());
    for _ in 0..4 {
        fib = (fib.1.clone(), [fib.1.clone(), fib.0].concat());
    }
    let t = &fib.1[..11];
    let b = optimal::brute_force(t);
    assert_eq!(b.decode().unwrap(), t);
    assert!(b.size() <= lz77(t).size());
    assert!(b.size() as f64 >= measures::delta(t));
}

/// Regression (fuzz OOM): an out-of-bounds copy of ~4 GB used to allocate the whole text
/// before its source was checked.
#[test]
fn rejects_huge_out_of_bounds_copy_without_allocating() {
    let scheme = MacroScheme::new(vec![Phrase::Copy {
        src: 4_268_053_047_640_996_667,
        len: 4_282_122_047,
    }]);

    assert_eq!(
        scheme.decode(),
        Err(InvalidScheme::SourceOutOfBounds { phrase: 0 })
    );
}

/// Phrase lengths that overflow `usize` are rejected instead of wrapping.
#[test]
fn rejects_overflowing_text_length() {
    let scheme = MacroScheme::new(vec![
        Phrase::Copy {
            src: 0,
            len: usize::MAX,
        },
        Phrase::Literal(b'a'),
    ]);

    assert_eq!(scheme.decode(), Err(InvalidScheme::TooLong));
}
