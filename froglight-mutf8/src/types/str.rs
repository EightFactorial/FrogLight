//! TODO

/// MUTF-8 string slices.
///
/// Equivalent to [`str`],
/// but uses MUTF-8 instead of UTF-8.
#[repr(transparent)]
pub struct MStr([u8]);
