//! TODO
#![expect(clippy::inline_always, reason = "Wrapper Functions")]

use facet::Facet;
use froglight_facet_iter::deserialize::DeserializeError;

use crate::types::indexed::{
    compound::IndexedCompound,
    core::{IndexCore, Ref},
};

/// TODO
///
/// # Errors
///
/// Returns an error if the value cannot be deserialized.
#[inline(always)]
pub fn from_nbt<T: Facet<'static>, C: IndexCore<Ref>>(
    _value: IndexedCompound<'_, Ref, C>,
) -> Result<T, DeserializeError> {
    todo!()
}

/// TODO
///
/// # Errors
///
/// Returns an error if the value cannot be deserialized.
#[inline(always)]
pub fn from_nbt_borrowed<'facet, T: Facet<'facet>, C: IndexCore<Ref> + 'facet>(
    _value: IndexedCompound<'facet, Ref, C>,
) -> Result<T, DeserializeError> {
    todo!()
}
