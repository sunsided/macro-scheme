//! Bidirectional macro schemes (Storer & Szymanski, 1982).
//!
//! A macro scheme parses a text into phrases, each either a literal
//! character or a copy of an equally long substring *anywhere* in the text,
//! earlier or later. It is valid if following copy pointers from every
//! position eventually reaches a literal; the smallest valid scheme has size
//! `b`, a fundamental (NP-hard) repetitiveness measure with
//! `δ ≤ γ ≤ b ≤ z` (substring complexity, attractor size, LZ77 phrases).
//!
//! ```mermaid
//! flowchart LR
//!   text[text] --> lz77["encode::lz77<br/>size z"]
//!   text --> lex["encode::lex_parse<br/>size ≤ 2r"]
//!   text --> opt["optimal::brute_force<br/>size b"]
//!   lz77 --> scheme[MacroScheme]
//!   lex --> scheme
//!   opt --> scheme
//!   scheme --> decode["decode / validate"]
//!   decode --> out["text or InvalidScheme"]
//! ```

#![forbid(unsafe_code)]

pub mod encode;
pub mod measures;
pub mod optimal;
mod suffix;

/// Doctests for the code examples in README.md (not part of the API).
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme_doctests {}

use std::fmt;

/// One phrase of a [`MacroScheme`]: a literal byte, or a copy of earlier or later text.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum Phrase {
    /// The byte itself, stored verbatim; every pointer chain must end at one.
    Literal(u8),
    /// Copy `len` bytes starting at text position `src`.
    ///
    /// The source may lie before or after the phrase, and may overlap it.
    Copy {
        /// Zero-based text position of the first copied byte.
        src: usize,
        /// Number of copied bytes; a valid scheme never has `0` here.
        len: usize,
    },
}

impl Phrase {
    /// Number of text bytes this phrase covers: `1` for a literal, `len` for a copy.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Phrase::Literal(_) => 1,
            Phrase::Copy { len, .. } => *len,
        }
    }

    /// Returns `true` for a zero-length copy, which no valid scheme contains.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Why a [`MacroScheme`] does not describe a text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidScheme {
    /// A copy phrase has `len == 0`.
    EmptyCopy {
        /// Index of the offending phrase in [`MacroScheme::phrases`].
        phrase: usize,
    },
    /// A copy's source range `src..src + len` reaches past the end of the text.
    SourceOutOfBounds {
        /// Index of the offending phrase in [`MacroScheme::phrases`].
        phrase: usize,
    },
    /// Following copy pointers from `position` never reaches a literal.
    Cycle {
        /// Text position that was revisited while following pointers.
        position: usize,
    },
    /// The phrase lengths add up to more than `usize::MAX`.
    TooLong,
}

impl fmt::Display for InvalidScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCopy { phrase } => write!(f, "phrase {phrase} copies zero bytes"),
            Self::SourceOutOfBounds { phrase } => {
                write!(f, "phrase {phrase} copies from outside the text")
            }
            Self::Cycle { position } => write!(f, "position {position} depends on itself"),
            Self::TooLong => write!(f, "the text length overflows usize"),
        }
    }
}

impl std::error::Error for InvalidScheme {}

/// A parsing of a text into [`Phrase`]s whose copies may point in either direction.
///
/// Construction does not validate; [`decode`](Self::decode) and
/// [`validate`](Self::validate) do.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct MacroScheme {
    /// The phrases in text order; phrase `i` starts where phrase `i - 1` ends.
    phrases: Vec<Phrase>,
}

impl MacroScheme {
    /// Wraps phrases without validating them; see [`validate`](Self::validate).
    #[must_use]
    pub fn new(phrases: Vec<Phrase>) -> Self {
        Self { phrases }
    }

    /// The phrases in text order.
    #[must_use]
    pub fn phrases(&self) -> &[Phrase] {
        &self.phrases
    }

    /// Number of phrases: the size measure being minimized.
    #[must_use]
    pub fn size(&self) -> usize {
        self.phrases.len()
    }

    /// Length of the text this scheme describes, saturating at `usize::MAX`.
    ///
    /// [`decode`](Self::decode) rejects schemes whose length overflows with
    /// [`InvalidScheme::TooLong`].
    #[must_use]
    pub fn text_len(&self) -> usize {
        self.phrases
            .iter()
            .fold(0, |n, p| n.saturating_add(p.len()))
    }

    /// Checks that the scheme describes a text, by decoding it.
    ///
    /// # Errors
    ///
    /// The same as [`decode`](Self::decode).
    pub fn validate(&self) -> Result<(), InvalidScheme> {
        self.decode().map(|_| ())
    }

    /// Reconstructs the text in `O(n)`: each position's copy pointer is
    /// followed until a literal is reached, memoizing along the way. A
    /// pointer chain that revisits a position is a cycle.
    ///
    /// Empty, out-of-bounds and overflowing copies are rejected before any
    /// buffer is allocated. Otherwise memory is proportional to
    /// [`text_len`](Self::text_len), even for a scheme that later fails with a
    /// cycle, so cap `text_len` before decoding untrusted schemes.
    ///
    /// # Errors
    ///
    /// - [`InvalidScheme::TooLong`] if the phrase lengths overflow `usize`.
    /// - [`InvalidScheme::EmptyCopy`] for a copy with `len == 0`.
    /// - [`InvalidScheme::SourceOutOfBounds`] for a copy reaching past the text.
    /// - [`InvalidScheme::Cycle`] if some position never reaches a literal.
    pub fn decode(&self) -> Result<Vec<u8>, InvalidScheme> {
        let n = self.check_phrases()?;
        let (bytes, marks, target) = self.place_phrases(n);

        resolve_pointers(bytes, marks, &target)
    }

    /// Checks every phrase against the text length before anything is allocated, so a
    /// malformed scheme cannot request a huge buffer. Returns the text length.
    fn check_phrases(&self) -> Result<usize, InvalidScheme> {
        let n = self
            .phrases
            .iter()
            .try_fold(0usize, |n, p| n.checked_add(p.len()))
            .ok_or(InvalidScheme::TooLong)?;

        for (i, p) in self.phrases.iter().enumerate() {
            if let Phrase::Copy { src, len } = *p {
                if len == 0 {
                    return Err(InvalidScheme::EmptyCopy { phrase: i });
                }

                if src.checked_add(len).is_none_or(|end| end > n) {
                    return Err(InvalidScheme::SourceOutOfBounds { phrase: i });
                }
            }
        }

        Ok(n)
    }

    /// Lays the phrases out over a text of `n` bytes, which [`check_phrases`](Self::check_phrases)
    /// returned.
    ///
    /// Returns the text bytes (only final at literal positions), a [`Mark`] per position, and
    /// per position the position it copies from (unused at literal positions).
    fn place_phrases(&self, n: usize) -> (Vec<u8>, Vec<Mark>, Vec<usize>) {
        let mut bytes = vec![0; n];
        let mut marks = vec![Mark::Pending; n];
        let mut target = vec![0; n];
        let mut pos = 0;

        for p in &self.phrases {
            match *p {
                Phrase::Literal(b) => {
                    bytes[pos] = b;
                    marks[pos] = Mark::Resolved;
                }
                Phrase::Copy { src, len } => {
                    for (offset, t) in target[pos..pos + len].iter_mut().enumerate() {
                        *t = src + offset;
                    }
                }
            }

            pos += p.len();
        }

        (bytes, marks, target)
    }
}

/// How far [`resolve_pointers`] has got with one text position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mark {
    /// A copied position whose byte is not known yet.
    Pending,
    /// On the pointer chain being followed right now; reaching it again is a cycle.
    OnPath,
    /// Its byte is final: a literal, or a copy whose chain reached one.
    Resolved,
}

/// Follows the copy pointers in `target` from every pending position to a literal,
/// writing the byte back along the whole chain so each position is walked once.
///
/// # Errors
///
/// [`InvalidScheme::Cycle`] if a chain revisits a position before reaching a literal.
fn resolve_pointers(
    mut bytes: Vec<u8>,
    mut marks: Vec<Mark>,
    target: &[usize],
) -> Result<Vec<u8>, InvalidScheme> {
    let mut path = Vec::new();

    for start in 0..bytes.len() {
        let mut cur = start;
        while marks[cur] != Mark::Resolved {
            if marks[cur] == Mark::OnPath {
                return Err(InvalidScheme::Cycle { position: cur });
            }

            marks[cur] = Mark::OnPath;
            path.push(cur);
            cur = target[cur];
        }

        let byte = bytes[cur];
        for p in path.drain(..) {
            bytes[p] = byte;
            marks[p] = Mark::Resolved;
        }
    }

    Ok(bytes)
}
