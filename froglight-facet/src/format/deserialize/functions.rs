//! TODO
#![expect(clippy::inline_always, reason = "Wrapper Functions")]

use facet::Facet;

use super::{Deserialize, DeserializeError};

/// TODO
///
/// # Errors
///
/// TODO
#[inline(always)]
pub fn from_slice<T: Facet<'static>>(slice: &[u8]) -> Result<T, DeserializeError> {
    <T as Deserialize>::from_slice(slice)
}

/// TODO
///
/// # Errors
///
/// TODO
#[inline(always)]
pub fn from_slice_remainder<T: Facet<'static>>(
    slice: &[u8],
) -> Result<(T, &[u8]), DeserializeError> {
    <T as Deserialize>::from_slice_remainder(slice)
}

/// TODO
///
/// # Errors
///
/// TODO
#[inline(always)]
pub fn from_slice_borrowed<'facet, T: Facet<'facet>>(
    slice: &'facet [u8],
) -> Result<(T, &'facet [u8]), DeserializeError> {
    <T as Deserialize>::from_slice_borrowed(slice)
}
