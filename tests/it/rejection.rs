//! Invalid schemes are rejected with the matching [`InvalidScheme`] variant.

use macro_scheme::{InvalidScheme, MacroScheme, Phrase};

/// Copies that point at each other never reach a literal.
#[test]
fn rejects_cycles() {
    // "ab" where each position copies the other: no literal is ever reached.
    let cyclic = MacroScheme::new(vec![
        Phrase::Copy { src: 1, len: 1 },
        Phrase::Copy { src: 0, len: 1 },
    ]);

    assert!(matches!(
        cyclic.validate(),
        Err(InvalidScheme::Cycle { .. })
    ));
}

/// A copy whose source ends past the text is out of bounds.
#[test]
fn rejects_bad_sources() {
    let out = MacroScheme::new(vec![Phrase::Literal(b'a'), Phrase::Copy { src: 2, len: 5 }]); // ends at 7 > 6

    assert_eq!(
        out.validate(),
        Err(InvalidScheme::SourceOutOfBounds { phrase: 1 })
    );
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

/// Zero-length copies are rejected.
#[test]
fn rejects_empty_copies() {
    let scheme = MacroScheme::new(vec![Phrase::Literal(b'a'), Phrase::Copy { src: 0, len: 0 }]);

    assert_eq!(scheme.decode(), Err(InvalidScheme::EmptyCopy { phrase: 1 }));
}
