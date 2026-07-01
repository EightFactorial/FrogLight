//! TODO
#![expect(clippy::inline_always, reason = "Wrapper Functions")]

use facet::Facet;

use crate::deserialize::{Deserialize, DeserializeError, future::DeserializeAsync};

/// TODO
///
/// # Errors
///
/// TODO
#[inline(always)]
pub fn from_slice<T: Facet<'static>>(slice: &[u8], protocol: u32) -> Result<T, DeserializeError> {
    <T as Deserialize>::from_slice(slice, false, protocol)
}

/// TODO
///
/// # Errors
///
/// TODO
#[inline(always)]
pub async fn from_slice_async<T: Facet<'static>>(
    slice: &[u8],
    protocol: u32,
) -> Result<T, DeserializeError> {
    <T as DeserializeAsync>::from_slice_async(slice, false, protocol).await
}

/// TODO
///
/// # Errors
///
/// TODO
#[inline(always)]
pub fn from_slice_variable<T: Facet<'static>>(
    slice: &[u8],
    protocol: u32,
) -> Result<T, DeserializeError> {
    <T as Deserialize>::from_slice(slice, true, protocol)
}

/// TODO
///
/// # Errors
///
/// TODO
#[inline(always)]
pub fn from_slice_remainder<T: Facet<'static>>(
    slice: &[u8],
    protocol: u32,
) -> Result<(T, &[u8]), DeserializeError> {
    <T as Deserialize>::from_slice_remainder(slice, false, protocol)
}

/// TODO
///
/// # Errors
///
/// TODO
#[inline(always)]
pub async fn from_slice_remainder_async<T: Facet<'static>>(
    slice: &[u8],
    protocol: u32,
) -> Result<(T, &[u8]), DeserializeError> {
    <T as DeserializeAsync>::from_slice_remainder_async(slice, false, protocol).await
}

/// TODO
///
/// # Errors
///
/// TODO
#[inline(always)]
pub fn from_slice_borrowed<'facet, T: Facet<'facet>>(
    slice: &'facet [u8],
    protocol: u32,
) -> Result<(T, &'facet [u8]), DeserializeError> {
    <T as Deserialize>::from_slice_borrowed(slice, false, protocol)
}
