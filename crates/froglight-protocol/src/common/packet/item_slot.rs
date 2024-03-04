use froglight_macros::FrogReadWrite;
use simdnbt::owned::Nbt;

use crate::common::NonZero;

/// An item slot.
///
/// This is used to represent an item slot in the inventory.
#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
// TODO: #[frog(tests = ["read_example"], bytes = [])]
pub struct ItemSlot {
    /// The kind of item in the slot.
    #[frog(var)]
    pub kind: u32,
    /// The number of items in the slot.
    pub count: NonZero<i8>,
    /// The item's data.
    pub data: Nbt,
}

impl ItemSlot {
    /// Creates a new item slot.
    ///
    /// # Panics
    /// Panics if `count` is [`i8::MIN`].
    #[must_use]
    #[inline]
    pub fn new(kind: u32, count: i8, data: Nbt) -> Self {
        Self { kind, count: NonZero::new_some(count - 1), data }
    }

    /// Returns `true` if the item slot is empty.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool { self.count.is_none() }

    /// Returns `true` if the item slot has NBT data.
    #[must_use]
    #[inline]
    pub fn has_nbt(&self) -> bool { self.data.is_some() }
}
