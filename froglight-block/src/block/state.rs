use crate::block::Block;

/// A unique identifier for a block state,
/// relative to all other blocks and states.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalId(u32);

impl GlobalId {
    /// Create a new [`GlobalId`].
    #[inline]
    #[must_use]
    pub const fn new(id: u32) -> Self { GlobalId(id) }

    /// Get the inner [`u32`] value.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> u32 { self.0 }
}

impl<T: Into<u32>> From<T> for GlobalId {
    fn from(value: T) -> Self { GlobalId(value.into()) }
}
impl From<Block> for GlobalId {
    fn from(block: Block) -> Self { block.global_id() }
}

impl<T: PartialEq<u32>> PartialEq<T> for GlobalId {
    fn eq(&self, other: &T) -> bool { other.eq(&self.0) }
}

// -------------------------------------------------------------------------------------------------

/// A unique identifier for a block state,
/// relative to all other states of the same block.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub struct StateId(u16);

impl StateId {
    /// Create a new [`StateId`].
    #[inline]
    #[must_use]
    pub const fn new(id: u16) -> Self { StateId(id) }

    /// Get the inner [`u16`] value.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> u16 { self.0 }
}

impl<T: Into<u16>> From<T> for StateId {
    fn from(value: T) -> Self { StateId(value.into()) }
}
impl From<Block> for StateId {
    fn from(block: Block) -> Self { block.state_id() }
}

impl<T: PartialEq<u16>> PartialEq<T> for StateId {
    fn eq(&self, other: &T) -> bool { other.eq(&self.0) }
}
