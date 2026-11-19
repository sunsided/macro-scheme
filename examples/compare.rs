//! Compare scheme sizes against the bounds. Run: `cargo run --example compare -- <text>`.

use macro_scheme::{encode, measures, optimal};

fn main() {
    let text = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "abaababaabaab".into());
    let t = text.as_bytes();
    println!("text: {text:?} (n = {})", t.len());
    println!("δ        = {:.3}", measures::delta(t));
    println!("z (LZ77) = {}", encode::lz77(t).size());
    println!("lex-parse = {}", encode::lex_parse(t).size());
    if t.len() <= 14 {
        let b = optimal::brute_force(t);
        println!("b (opt)  = {}  {:?}", b.size(), b.phrases());
    } else {
        println!("b (opt)  = skipped (brute force is exponential; n > 14)");
    }
}
