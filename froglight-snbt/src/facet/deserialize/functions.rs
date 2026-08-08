//! TODO
#![expect(clippy::inline_always, reason = "Wrapper Functions")]

use facet::Facet;
use froglight_facet_iter::deserialize::DeserializeError;

use crate::types::indexed::{compound::IndexedCompound, core::IndexCore};

/// TODO
///
/// # Errors
///
/// Returns an error if the value cannot be deserialized.
#[inline(always)]
pub fn from_snbt<T: Facet<'static>, C: IndexCore>(
    _value: IndexedCompound<'_, C>,
) -> Result<T, DeserializeError> {
    todo!()
}

/// TODO
///
/// # Errors
///
/// Returns an error if the value cannot be deserialized.
#[inline(always)]
pub fn from_snbt_borrowed<'facet, T: Facet<'facet>, C: IndexCore + 'facet>(
    _value: IndexedCompound<'facet, C>,
) -> Result<T, DeserializeError> {
    todo!()
}
