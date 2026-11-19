//! Integration tests, compiled as a single test binary.

mod fixtures;
mod properties;
mod rejection;
mod strategies;

/// Converts a scheme size to `f64` for comparisons with [`macro_scheme::measures::delta`].
///
/// # Panics
///
/// If `n` does not fit in `u32`, which test inputs never reach.
fn as_f64(n: usize) -> f64 {
    f64::from(u32::try_from(n).expect("test sizes fit in u32"))
}
