/// A unique identifier for an item,
/// relative to all other items.
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
// impl From<Item> for GlobalId {
//     fn from(item: Item) -> Self { item.global_id() }
// }

impl<T: PartialEq<u32>> PartialEq<T> for GlobalId {
    fn eq(&self, other: &T) -> bool { other.eq(&self.0) }
}
