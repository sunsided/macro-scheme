//! Bidirectional macro schemes (Storer & Szymanski, 1982).
//!
//! A macro scheme parses a text into phrases, each either a literal
//! character or a copy of an equally long substring *anywhere* in the text,
//! earlier or later. It is valid if following copy pointers from every
//! position eventually reaches a literal; the smallest valid scheme has size
//! `b`, a fundamental (NP-hard) repetitiveness measure with
//! `δ ≤ γ ≤ b ≤ z` (substring complexity, attractor size, LZ77 phrases).

pub mod encode;
pub mod measures;
pub mod optimal;
mod suffix;

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum Phrase {
    Literal(u8),
    /// Copy `len` bytes starting at text position `src`.
    Copy {
        src: usize,
        len: usize,
    },
}

impl Phrase {
    pub fn len(&self) -> usize {
        match self {
            Phrase::Literal(_) => 1,
            Phrase::Copy { len, .. } => *len,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidScheme {
    EmptyCopy {
        phrase: usize,
    },
    SourceOutOfBounds {
        phrase: usize,
    },
    /// Following copy pointers from `position` never reaches a literal.
    Cycle {
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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct MacroScheme {
    phrases: Vec<Phrase>,
}

impl MacroScheme {
    /// Wraps phrases without validating them; see [`validate`](Self::validate).
    pub fn new(phrases: Vec<Phrase>) -> Self {
        Self { phrases }
    }

    pub fn phrases(&self) -> &[Phrase] {
        &self.phrases
    }

    /// Number of phrases: the size measure being minimized.
    pub fn size(&self) -> usize {
        self.phrases.len()
    }

    /// Length of the text this scheme describes.
    pub fn text_len(&self) -> usize {
        self.phrases.iter().map(Phrase::len).sum()
    }

    pub fn validate(&self) -> Result<(), InvalidScheme> {
        self.decode().map(|_| ())
    }

    /// Reconstructs the text in `O(n)`: each position's copy pointer is
    /// followed until a literal is reached, memoizing along the way. A
    /// pointer chain that revisits a position is a cycle.
    pub fn decode(&self) -> Result<Vec<u8>, InvalidScheme> {
        const LITERAL: usize = usize::MAX;
        let n = self.check_phrases()?;
        let mut out: Vec<Option<u8>> = vec![None; n];
        let mut target = vec![LITERAL; n];
        let mut pos = 0;
        for p in &self.phrases {
            match *p {
                Phrase::Literal(b) => out[pos] = Some(b),
                Phrase::Copy { src, len } => {
                    for o in 0..len {
                        target[pos + o] = src + o;
                    }
                }
            }
            pos += p.len();
        }

        let mut on_path = vec![false; n];
        let mut path = Vec::new();
        for start in 0..n {
            let mut cur = start;
            while out[cur].is_none() {
                if on_path[cur] {
                    return Err(InvalidScheme::Cycle { position: cur });
                }
                on_path[cur] = true;
                path.push(cur);
                cur = target[cur];
            }
            let byte = out[cur];
            for p in path.drain(..) {
                out[p] = byte;
                on_path[p] = false;
            }
        }
        Ok(out
            .into_iter()
            .map(|b| b.expect("all positions resolved"))
            .collect())
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
}
