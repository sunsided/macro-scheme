---
applyTo: "**/*.md,docs/**"
---

# Documentation

- README.md is rendered on crates.io and docs.rs. Every link to a repo file and every
  image uses an absolute URL (`https://github.com/sunsided/macro-scheme/...`). Only in-page
  anchors may be relative.
- Mermaid diagrams (```` ```mermaid ````) must be valid (`task lint:mermaid` renders them)
  and must match the code. Keep each diagram under about 15 nodes.
- Code blocks are tagged with their language. Rust blocks in README.md run as doctests
  (`readme_doctests` in `src/lib.rs`), so they must compile.
- Keep badges, the License section and the MSRV statement consistent with `Cargo.toml`.
