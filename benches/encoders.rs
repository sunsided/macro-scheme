//! Benchmarks for the encoders and the decoder on a highly repetitive text.

use std::hint::black_box;

use criterion::Criterion;
use macro_scheme::encode::{lex_parse, lz77};

/// Length of the benchmark text in bytes.
const TEXT_LEN: usize = 2048;

/// Runs every benchmark in this file with criterion's command-line flags.
fn main() {
    let mut criterion = Criterion::default().configure_from_args();

    bench_encoders(&mut criterion);
    bench_decode(&mut criterion);
    criterion.final_summary();
}

/// A prefix of the Fibonacci word, a classic highly repetitive input, of `len` bytes.
fn fibonacci_word(len: usize) -> Vec<u8> {
    let (mut shorter, mut longer) = (b"a".to_vec(), b"ab".to_vec());
    while longer.len() < len {
        (shorter, longer) = (longer.clone(), [longer, shorter].concat());
    }

    longer.truncate(len);
    longer
}

/// Benchmarks [`lz77`] and [`lex_parse`] on the same text.
fn bench_encoders(criterion: &mut Criterion) {
    let text = fibonacci_word(TEXT_LEN);

    criterion.bench_function("lz77", |b| b.iter(|| lz77(black_box(&text))));
    criterion.bench_function("lex_parse", |b| b.iter(|| lex_parse(black_box(&text))));
}

/// Benchmarks decoding the forward-pointing lex-parse scheme of the text.
fn bench_decode(criterion: &mut Criterion) {
    let scheme = lex_parse(&fibonacci_word(TEXT_LEN));

    criterion.bench_function("decode", |b| b.iter(|| black_box(&scheme).decode()));
}
