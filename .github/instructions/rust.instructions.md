---
applyTo: "**/*.rs"
---

# Rust code

- Every module (`//!`), type, field, variant, function, constant and macro has a doc
  comment, private and `pub(crate)` items included. Docs state intent, invariants, units and
  error/panic conditions (`# Errors`, `# Panics`). They do not restate the name.
- No `mod.rs`: modules with children are `foo.rs` plus `foo/`. Integration tests live under
  `tests/it/`.
- No separator comments (`// -----`, `// =====`). If a file needs sections, split it into
  submodules.
- Leave a blank line after the closing `}` of a block statement when another statement
  follows, and between logical phases of a function.
- One level of abstraction per function. Search and decode loops are short drivers that call
  documented step functions. If a block needs a "what" comment, extract it into a function
  named after that comment.
- `clippy::pedantic` clean. Suppression only with a scoped
  `#[expect(lint, reason = "...")]`. Never `#[allow]`, and never crate-level suppression.
- No `unsafe`: the crate root has `#![forbid(unsafe_code)]`.
- `MacroScheme::decode` takes untrusted phrase lists: it rejects empty, out-of-bounds and
  overflowing copies before it allocates, allocates memory proportional to `text_len()`
  otherwise, and never panics. Changes to it keep the `decode` fuzz target and the
  `decode_never_panics` property passing.
- Every encoder returns a valid scheme that decodes to its input; add a round-trip property
  in `tests/it/properties.rs` for each new encoder.
- The mermaid diagram in the `lib.rs` crate docs must match the modules it shows.
