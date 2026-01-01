/// A unique identifier for a block state,
/// relative to all other blocks and states.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalId(u32);

impl GlobalId {
    /// Create a new [`GlobalId`].
    #[inline]
    #[must_use]
    pub const fn new(id: u32) -> Self { GlobalId(id) }

    /// Return the inner [`u32`] value.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> u32 { self.0 }
}

impl<T: Into<u32>> From<T> for GlobalId {
    fn from(value: T) -> Self { GlobalId(value.into()) }
}

// -------------------------------------------------------------------------------------------------

/// A unique identifier for a block state,
/// relative to all other states of the same block.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateId(u16);

impl StateId {
    /// Create a new [`StateId`].
    #[inline]
    #[must_use]
    pub const fn new(id: u16) -> Self { StateId(id) }

    /// Return the inner [`u16`] value.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> u16 { self.0 }
}

impl<T: Into<u16>> From<T> for StateId {
    fn from(value: T) -> Self { StateId(value.into()) }
}
