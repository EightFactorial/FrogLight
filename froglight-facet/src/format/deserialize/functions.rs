//! TODO
#![expect(clippy::inline_always, reason = "Wrapper Functions")]
#![expect(clippy::result_unit_err, reason = "WIP")]

use super::Deserialize;

/// TODO
///
/// # Errors
///
/// TODO
#[inline(always)]
pub fn from_slice<T: Deserialize<'static>>(slice: &[u8]) -> Result<T, ()> {
    <T as Deserialize>::from_slice(slice)
}

/// TODO
///
/// # Errors
///
/// TODO
#[inline(always)]
pub fn from_slice_remainder<T: Deserialize<'static>>(slice: &[u8]) -> Result<(T, &[u8]), ()> {
    <T as Deserialize>::from_slice_remainder(slice)
}

/// TODO
///
/// # Errors
///
/// TODO
#[inline(always)]
pub fn from_slice_borrowed<'facet, T: Deserialize<'facet>>(
    slice: &'facet [u8],
) -> Result<(T, &'facet [u8]), ()> {
    <T as Deserialize>::from_slice_borrowed(slice)
}
