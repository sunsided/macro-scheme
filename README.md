# macro-scheme

[![crates.io](https://img.shields.io/crates/v/macro-scheme.svg)](https://crates.io/crates/macro-scheme)
[![docs.rs](https://img.shields.io/docsrs/macro-scheme)](https://docs.rs/macro-scheme)
[![CI](https://github.com/sunsided/macro-scheme/actions/workflows/ci.yml/badge.svg)](https://github.com/sunsided/macro-scheme/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/sunsided/macro-scheme/graph/badge.svg)](https://codecov.io/gh/sunsided/macro-scheme)
[![MSRV](https://img.shields.io/crates/msrv/macro-scheme)](https://crates.io/crates/macro-scheme)
[![License](https://img.shields.io/crates/l/macro-scheme)](https://github.com/sunsided/macro-scheme#license)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Bidirectional macro schemes: LZ77-like parsings whose copies may point **forward** as well as backward, valid when every pointer chain ends at a literal. The smallest scheme size `b` is an NP-hard repetitiveness measure, with `δ ≤ γ ≤ b ≤ z`.

```bash
cargo test
cargo run --example compare -- abaababaabaab
# z (LZ77) = 6, lex-parse = 5, b (opt) = 4  [Copy { src: 5, len: 6 }, …]  ← first phrase points right
```

## Usage

```rust
use macro_scheme::encode::lex_parse;
use macro_scheme::optimal::brute_force;

let text = b"abaababaabaab";

// Lex-parse copies may point to the right, and still decode.
let scheme = lex_parse(text);
assert_eq!(scheme.decode().unwrap(), text);

// The optimum is never larger than either encoder.
assert!(brute_force(text).size() <= scheme.size());
```

| Module | Contents |
|---|---|
| `MacroScheme` | `validate` and `decode` in O(n), with cycle detection on the pointer graph |
| `encode` | `lz77` (left-only, size `z`) and `lex_parse` (Navarro, Ochoa & Prezza, size ≤ 2r) |
| `measures` | `delta`, `z` |
| `optimal` | branch-and-bound exact `b`, an oracle for n ≲ 12 |

## How it works

A scheme is a list of phrases that cover the text from left to right. `Literal(b)` stores one byte. `Copy { src, len }` says that the next `len` bytes equal the `len` bytes starting at position `src`. That source may lie before the phrase, after it, or overlap it.

The optimal scheme for `abaababaabaab` has four phrases:

```text
position  0 1 2 3 4 5 | 6 | 7 | 8 9 10 11 12
text      a b a a b a | b | a | a b a  a  b
phrase    Copy{5,6}   | b | a | Copy{0,5}
```

**Decoding.** A copied position stores no byte. It only says "my byte is the byte at position X". If position `p` lies in a copy phrase that starts at position `start` and copies from `src`, then X is `src + (p - start)`. To find the byte at `p`, decoding looks up X. If X is also a copied position, it looks up X's source, and so on until it reaches a literal. For position 0:

| Step | Position | Phrase it lies in | Offset in phrase | Copies from |
|---|---|---|---|---|
| 1 | 0 | `Copy{5,6}`, starts at 0 | 0 | 5 + 0 = 5 |
| 2 | 5 | `Copy{5,6}`, starts at 0 | 5 | 5 + 5 = 10 |
| 3 | 10 | `Copy{0,5}`, starts at 8 | 2 | 0 + 2 = 2 |
| 4 | 2 | `Copy{5,6}`, starts at 0 | 2 | 5 + 2 = 7 |
| 5 | 7 | `Literal(a)` | | stop: the byte is `a` |

So positions 0, 5, 10 and 2 all hold `a`. The chain jumps right, right, left and right again, which is why decoding cannot simply read from left to right. Once a chain reaches its literal, the byte is written to every position on the way, so each position is looked up only once and decoding is `O(n)`. If a chain comes back to a position it already passed, it can never reach a literal, and the scheme is rejected as a `Cycle`.

**Encoders.** `lz77` only copies from the left, taking the longest earlier match each time. Its pointers always move left, so it cannot form a cycle; its size is `z`. `lex_parse` sorts all suffixes and lets each phrase copy from the suffix just before it in sorted order, for as long as the two agree. Its pointers can move right, but they always lead to a lexicographically smaller suffix, so they cannot loop either.

**Optimum.** `brute_force` starts from the smaller of the two encoder results. It then tries every parsing depth-first, longest copies first. It drops any branch that cannot beat the best scheme so far, and keeps a finished candidate only if it decodes. This is exponential, so use it only on tiny inputs and as a reference for the encoders.

## Verification
- Round-trip property tests for every encoder.
- The hierarchy `δ ≤ b_opt ≤ min(z, lex-parse)` is checked on random strings.
- Cycle and out-of-bounds rejection tests.

## Roadmap
1. SA-IS suffix arrays (`libsais`) and an SA-based LZ77 factorizer; compact serialization.
2. LZRR (Nishimoto & Tabei), plus computing `r` (BWT runs) and `γ` bounds.
3. A MaxSAT exact solver via `rustsat` / WCNF export (Bannai et al., ESA 2022).
4. Research: local search between lex-parse and the optimum.

## MSRV

The minimum supported Rust version is 1.85 (edition 2024). It is raised in a minor release when a dependency or a language feature needs it.

## Contributing

See [CONTRIBUTING.md](https://github.com/sunsided/macro-scheme/blob/main/CONTRIBUTING.md).

## License

Licensed under either of [Apache License, Version 2.0](https://github.com/sunsided/macro-scheme/blob/main/LICENSES/Apache-2.0.txt) or [MIT license](https://github.com/sunsided/macro-scheme/blob/main/LICENSES/MIT.txt) at your option (`MIT OR Apache-2.0`).

The code under [`examples/`](https://github.com/sunsided/macro-scheme/tree/main/examples) is dedicated to the public domain under [CC0-1.0](https://github.com/sunsided/macro-scheme/blob/main/LICENSES/CC0-1.0.txt), so you can copy it into anything.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
