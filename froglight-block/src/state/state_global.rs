//! TODO

#[cfg(feature = "facet")]
use froglight_facet as mc;

use crate::{block::Block, prelude::BlockVersion};

/// A unique identifier for a block state,
/// relative to all other blocks and states in the same version.
///
/// This only guarantees uniqueness if both blocks are, for example,
/// from [`V26_1`](froglight_common::prelude::V26_1).
///
/// Two blocks of the same type and different states,
/// like stair orientation, *will not* equal each other.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", facet(transparent, mc::variable_inner))]
pub struct GlobalStateId(u32);

impl GlobalStateId {
    /// Create a new [`GlobalStateId`].
    #[inline]
    #[must_use]
    pub const fn new(id: u32) -> Self { GlobalStateId(id) }

    /// Try to convert this [`GlobalStateId`] into a [`Block`].
    ///
    /// See [`BlockStorage::get_block_by_state`](crate::storage::BlockStorage::get_block_by_state).
    #[inline]
    #[must_use]
    pub fn try_into_block<V: BlockVersion>(self) -> Option<Block> {
        V::blocks().get_block_by_state(self)
    }

    /// Get the inner [`u32`] value.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> u32 { self.0 }
}

impl<T: Into<u32>> From<T> for GlobalStateId {
    fn from(value: T) -> Self { GlobalStateId(value.into()) }
}
impl From<Block> for GlobalStateId {
    fn from(value: Block) -> Self { value.global_id() }
}

impl<T: PartialEq<u32>> PartialEq<T> for GlobalStateId {
    fn eq(&self, other: &T) -> bool { other.eq(&self.0) }
}
