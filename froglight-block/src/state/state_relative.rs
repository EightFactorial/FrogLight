//! TODO

#[cfg(feature = "facet")]
use froglight_facet as mc;

use crate::{
    block::Block,
    prelude::{BlockType, BlockVersion},
};

/// A unique identifier for a block state,
/// relative to all other states of the same block in the same version.
///
/// This only guarantees uniqueness if both blocks are, for example,
/// [`Dirt`](crate::prelude::block::Dirt) from
/// [`V26_1`](froglight_common::prelude::V26_1).
///
/// Two blocks of the same type and different states,
/// like stair orientation, *will not* equal each other.
///
/// # Note
///
/// The only benefit of [`RelativeStateId`] over [`GlobalStateId`] is that it is
/// smaller. For almost all operations the cost of making sure both blocks are
/// of the same type is more expensive than using [`GlobalStateId`] in the first
/// place.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", facet(transparent, mc::variable_inner))]
pub struct RelativeStateId(u16);

impl RelativeStateId {
    /// Create a new [`RelativeStateId`].
    #[inline]
    #[must_use]
    pub const fn new(id: u16) -> Self { RelativeStateId(id) }

    /// Try to create a [`Block`] using this [`RelativeStateId`].
    ///
    /// See [`Block::try_new_from`] for more information.
    #[inline]
    #[must_use]
    pub fn try_as_block<B: BlockType<V>, V: BlockVersion>(self) -> Option<Block> {
        Block::try_new_from::<B, V>(self)
    }

    /// Get the inner [`u16`] value.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> u16 { self.0 }
}

impl<T: Into<u16>> From<T> for RelativeStateId {
    fn from(value: T) -> Self { RelativeStateId(value.into()) }
}
impl From<Block> for RelativeStateId {
    fn from(value: Block) -> Self { value.state_id() }
}

impl<T: PartialEq<u16>> PartialEq<T> for RelativeStateId {
    fn eq(&self, other: &T) -> bool { other.eq(&self.0) }
}
