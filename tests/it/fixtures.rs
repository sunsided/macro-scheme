//! Known inputs with known properties.

use crate::as_f64;
use macro_scheme::encode::lz77;
use macro_scheme::{measures, optimal};

/// The optimum on a Fibonacci-word prefix decodes correctly and lies between δ and z.
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
    assert!(as_f64(b.size()) >= measures::delta(t));
}

/// The optimum matches hand-checked values, so over-eager pruning cannot fall back to an
/// encoder's larger scheme unnoticed.
#[test]
fn optimum_has_known_size() {
    for (text, b) in [
        (&b"abaababaabaab"[..], 4),
        (b"abacabadabacab", 7),
        (b"aaaa", 2),
    ] {
        assert_eq!(optimal::brute_force(text).size(), b, "{text:?}");
    }
}
