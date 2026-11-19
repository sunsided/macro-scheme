# macro-scheme

[![crates.io](https://img.shields.io/crates/v/macro-scheme.svg)](https://crates.io/crates/macro-scheme)
[![docs.rs](https://img.shields.io/docsrs/macro-scheme)](https://docs.rs/macro-scheme)
[![CI](https://github.com/sunsided/macro-scheme/actions/workflows/ci.yml/badge.svg)](https://github.com/sunsided/macro-scheme/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/sunsided/macro-scheme/graph/badge.svg)](https://codecov.io/gh/sunsided/macro-scheme)
[![MSRV](https://img.shields.io/crates/msrv/macro-scheme)](https://crates.io/crates/macro-scheme)
[![License](https://img.shields.io/crates/l/macro-scheme)](https://github.com/sunsided/macro-scheme#license)

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
