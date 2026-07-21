//! TODO
#![expect(clippy::inline_always, reason = "Wrapper Functions")]

use facet::Facet;
use froglight_facet_iter::serialize::SerializeError;

use crate::prelude::*;

/// TODO
///
/// # Errors
///
/// Returns an error if the value cannot be serialized.
#[inline(always)]
pub fn to_nbt<'facet, T: Facet<'facet>>(_value: &T) -> Result<Nbt, SerializeError> { todo!() }
