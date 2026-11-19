# macro-scheme

Bidirectional macro schemes: LZ77-like parsings whose copies may point **forward** as well as backward, valid when every pointer chain ends at a literal. The smallest scheme size `b` is an NP-hard repetitiveness measure, with `δ ≤ γ ≤ b ≤ z`.

```bash
cargo test
cargo run --example compare -- abaababaabaab
# z (LZ77) = 6, lex-parse = 5, b (opt) = 4  [Copy { src: 5, len: 6 }, …]  ← first phrase points right
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

## License

Licensed under either of [Apache License, Version 2.0](https://github.com/sunsided/macro-scheme/blob/main/LICENSES/Apache-2.0.txt) or [MIT license](https://github.com/sunsided/macro-scheme/blob/main/LICENSES/MIT.txt) at your option (`MIT OR Apache-2.0`).

The code under [`examples/`](https://github.com/sunsided/macro-scheme/tree/main/examples) is dedicated to the public domain under [CC0-1.0](https://github.com/sunsided/macro-scheme/blob/main/LICENSES/CC0-1.0.txt), so you can copy it into anything.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
