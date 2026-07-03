//! TODO

#[cfg(feature = "facet")]
use froglight_facet as mc;

use crate::{block::Block, prelude::BlockVersion};

/// A unique identifier for a block type id,
/// relative to all other blocks in the same version.
///
/// This only guarantees uniqueness if both blocks are, for example,
/// from [`V26_1`](froglight_common::prelude::V26_1).
///
/// Two blocks of the same type and different states,
/// like stair orientation, *will* equal each other.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", facet(transparent, mc::variable_inner))]
pub struct GlobalBlockId(u16);

impl GlobalBlockId {
    /// Create a new [`GlobalBlockId`].
    #[inline]
    #[must_use]
    pub const fn new(id: u16) -> Self { GlobalBlockId(id) }

    /// Try to convert this [`GlobalBlockId`] into a [`Block`].
    ///
    /// See [`BlockStorage::get_block_by_id`](crate::storage::BlockStorage::get_block_by_id).
    #[inline]
    #[must_use]
    pub fn try_into_block<V: BlockVersion>(self) -> Option<Block> {
        V::blocks().get_block_by_id(self)
    }

    /// Get the inner [`u16`] value.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> u16 { self.0 }
}

impl<T: Into<u16>> From<T> for GlobalBlockId {
    fn from(value: T) -> Self { GlobalBlockId(value.into()) }
}
impl From<Block> for GlobalBlockId {
    fn from(_value: Block) -> Self { GlobalBlockId(0) }
}

impl<T: PartialEq<u16>> PartialEq<T> for GlobalBlockId {
    fn eq(&self, other: &T) -> bool { other.eq(&self.0) }
}
