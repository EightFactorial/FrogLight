//! TODO
#![expect(clippy::inline_always, reason = "Wrapper Functions")]

use facet::Facet;
use froglight_facet_iter::deserialize::DeserializeError;

use crate::{
    facet::deserialize::DeserializeNbt,
    types::indexed::{
        IndexedNbt,
        core::{Ref, SliceCore},
    },
};

/// TODO
///
/// # Errors
///
/// Returns an error if the value cannot be deserialized.
#[inline(always)]
pub fn from_nbt<T: Facet<'static>>(
    nbt: &IndexedNbt<SliceCore<'_, Ref>>,
) -> Result<T, DeserializeError> {
    <T as DeserializeNbt>::from_nbt(nbt)
}

/// TODO
///
/// # Errors
///
/// Returns an error if the value cannot be deserialized.
#[inline(always)]
pub fn from_nbt_borrowed<'facet, T: Facet<'facet>>(
    nbt: &IndexedNbt<SliceCore<'facet, Ref>>,
) -> Result<T, DeserializeError> {
    <T as DeserializeNbt>::from_nbt_borrowed(nbt)
}
