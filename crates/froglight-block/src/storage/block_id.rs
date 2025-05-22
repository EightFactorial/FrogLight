#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, Into};

/// A raw block id that represents a type of block.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Into, Deref)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
pub struct GlobalBlockId(u32);

impl GlobalBlockId {
    /// Create a new [`GlobalBlockId`] with the given id.
    ///
    /// # Warning
    /// There is no guarantee that the given id is valid or represents the
    /// same block between versions. Ids may even change between program runs!
    #[inline]
    #[must_use]
    pub const fn new_unchecked(block: u32) -> Self { Self(block) }
}

// -------------------------------------------------------------------------------------------------

/// A block id that is relative to it's state range.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Into, Deref)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
pub(crate) struct RelativeBlockState(u16);

impl RelativeBlockState {
    /// Create a new [`RelativeBlockState`] with the given id.
    ///
    /// # Warning
    /// There is no guarantee that the given id is valid or represents the
    /// same state between versions. Ids may even change between program runs!
    #[inline]
    #[must_use]
    pub(crate) const fn new_unchecked(state: u16) -> Self { Self(state) }
}

impl From<u32> for RelativeBlockState {
    #[cfg(debug_assertions)]
    fn from(state: u32) -> Self {
        Self(u16::try_from(state).expect("RelativeBlockState is too large!"))
    }

    #[inline]
    #[cfg(not(debug_assertions))]
    fn from(state: u32) -> Self { Self(state as u16) }
}
impl From<RelativeBlockState> for u32 {
    #[inline]
    fn from(state: RelativeBlockState) -> Self { u32::from(state.0) }
}

impl From<usize> for RelativeBlockState {
    #[cfg(debug_assertions)]
    fn from(state: usize) -> Self {
        Self(u16::try_from(state).expect("RelativeBlockState is too large!"))
    }

    #[inline]
    #[cfg(not(debug_assertions))]
    fn from(state: usize) -> Self { Self(state as u16) }
}
impl From<RelativeBlockState> for usize {
    #[inline]
    fn from(state: RelativeBlockState) -> Self { usize::from(state.0) }
}
