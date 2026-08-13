//! TODO
#![expect(clippy::inline_always, reason = "Wrapper Functions")]

use facet::Facet;
use froglight_facet_iter::deserialize::DeserializeError;

use crate::{
    facet::deserialize::DeserializeSnbt,
    types::indexed::{IndexedSnbt, core::SliceCore},
};

/// TODO
///
/// # Errors
///
/// Returns an error if the string is not valid SNBT,
/// or if the deserialization fails.
#[inline(always)]
pub fn from_snbt_string<T: Facet<'static>>(string: &str) -> Result<T, DeserializeError> {
    <T as DeserializeSnbt>::from_snbt_string(string)
}

/// TODO
///
/// # Errors
///
/// Returns an error if the value cannot be deserialized.
#[inline(always)]
pub fn from_snbt<T: Facet<'static>>(
    snbt: &IndexedSnbt<SliceCore<'_>>,
) -> Result<T, DeserializeError> {
    <T as DeserializeSnbt>::from_snbt(snbt)
}

/// TODO
///
/// # Errors
///
/// Returns an error if the value cannot be deserialized.
#[inline(always)]
pub fn from_snbt_borrowed<'facet, T: Facet<'facet>>(
    snbt: &'facet IndexedSnbt<SliceCore<'facet>>,
) -> Result<T, DeserializeError> {
    <T as DeserializeSnbt>::from_snbt_borrowed(snbt)
}
