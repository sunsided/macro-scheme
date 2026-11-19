# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/sunsided/macro-scheme/releases/tag/v0.1.0) - 2026-11-19

### Added

- Add the unsafe-forbidden badge to the README
- Add release-plz and tag-triggered crates.io publishing
- Add CONTRIBUTING.md
- Add git hooks and Copilot review instructions
- Add crate keywords, categories and README badges
- Add a decode fuzz target, robustness properties and benchmarks
- Add cargo-deny, Dependabot and typos configuration
- Add coverage reporting to Codecov
- Add Taskfile and harden the CI workflows
- Add README
- Add an example comparing the encoders
- Add property tests
- Add an exact minimizer for tiny inputs
- Add repetitiveness measures
- Add LZ77 and lex-parse encoders
- Add macro schemes with validation and decoding

### Changed

- Accept hyphenated -ed imperatives in the commit-msg hook
- Show the mermaid hook's skip note
- Retry and time-limit the crates.io version check
- Accept more -ed imperatives in the commit-msg hook
- Point config comments at files that exist in this repository
- Assert decode's resolution invariant instead of dropping gaps
- Skip the mermaid hook when mermaid-cli is not installed
- Fail the publish job on unexpected crates.io responses
- Install actionlint from its release instead of install-action
- Document decode's memory bound precisely
- Extract helpers in the encoders and the exact minimizer
- Document every item and clear the pedantic findings
- Enable pedantic clippy, doc-comment lints and forbid unsafe code
- Render mermaid diagrams on docs.rs and doctest the README
- Move license texts to LICENSES/ and add REUSE.toml
- Initial commit

### Fixed

- Fix the fuzz CI job building for the musl target
- Fix decode allocating the text before validating copies
