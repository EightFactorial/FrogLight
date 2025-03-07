#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

/// A raw item id that represents a type of item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
pub struct GlobalItemId(u32);

impl GlobalItemId {
    /// Create a new [`GlobalItemId`] with the given id.
    ///
    /// # Warning
    /// There is no guarantee that the given id is valid or represents the
    /// same item between versions. Ids may even change between program runs!
    #[inline]
    #[must_use]
    pub const fn new_unchecked(item: u32) -> Self { Self(item) }
}

impl From<GlobalItemId> for u32 {
    fn from(id: GlobalItemId) -> Self { id.0 }
}

impl std::ops::Deref for GlobalItemId {
    type Target = u32;

    fn deref(&self) -> &Self::Target { &self.0 }
}
