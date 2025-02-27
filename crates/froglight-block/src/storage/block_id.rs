#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

/// A raw block id that represents a type of block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
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

impl From<GlobalBlockId> for u32 {
    fn from(id: GlobalBlockId) -> Self { id.0 }
}

impl std::ops::Deref for GlobalBlockId {
    type Target = u32;

    fn deref(&self) -> &Self::Target { &self.0 }
}

/// A block id that is relative to it's state range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
pub(crate) struct RelativeBlockState(u16);

impl From<u16> for RelativeBlockState {
    fn from(state: u16) -> Self { Self(state) }
}
impl From<RelativeBlockState> for u16 {
    fn from(state: RelativeBlockState) -> Self { state.0 }
}

impl From<u32> for RelativeBlockState {
    fn from(state: u32) -> Self {
        Self(
            #[cfg(debug_assertions)]
            u16::try_from(state).expect("RelativeBlockState is too large!"),
            #[cfg(not(debug_assertions))]
            #[expect(clippy::cast_possible_truncation)]
            {
                state as u16
            },
        )
    }
}
impl From<RelativeBlockState> for u32 {
    fn from(state: RelativeBlockState) -> Self { u32::from(state.0) }
}

impl From<usize> for RelativeBlockState {
    fn from(state: usize) -> Self {
        Self(
            #[cfg(debug_assertions)]
            u16::try_from(state).expect("RelativeBlockState is too large!"),
            #[cfg(not(debug_assertions))]
            #[expect(clippy::cast_possible_truncation)]
            {
                state as u16
            },
        )
    }
}
impl From<RelativeBlockState> for usize {
    fn from(state: RelativeBlockState) -> Self { usize::from(state.0) }
}

impl std::ops::Deref for RelativeBlockState {
    type Target = u16;

    fn deref(&self) -> &Self::Target { &self.0 }
}
