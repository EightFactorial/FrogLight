//! TODO

use std::num::NonZeroU8;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_common::version::Version;
use froglight_item::item::UntypedItem;

#[cfg(feature = "io")]
mod component;
#[cfg(feature = "io")]
pub use component::InventoryComponents;

#[cfg(feature = "io")]
mod raw;
#[cfg(feature = "io")]
pub use raw::RawInventorySlot;

/// A slot in an inventory.
///
/// May contain an item or be empty.
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Default, PartialEq))]
pub struct InventorySlot<V: Version>(Option<(NonZeroU8, UntypedItem<V>)>);

impl<V: Version> InventorySlot<V> {
    /// Create a new empty [`InventorySlot`].
    #[must_use]
    pub const fn new_empty() -> Self { Self(None) }

    /// Create a new [`InventorySlot`] with the given item.
    #[must_use]
    pub const fn new_from(count: NonZeroU8, item: UntypedItem<V>) -> Self {
        Self(Some((count, item)))
    }

    /// Returns `true` if the slot is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.0.is_none() }

    /// Get the amount of items in the slot.
    #[must_use]
    pub fn count(&self) -> u8 { self.0.as_ref().map_or(0, |(count, _)| count.get()) }

    /// Get the amount of items in the slot mutably.
    #[must_use]
    pub fn count_mut(&mut self) -> Option<&mut NonZeroU8> {
        self.0.as_mut().map(|(count, _)| count)
    }

    /// Get the [`UntypedItem`] in the slot.
    #[must_use]
    pub fn item(&self) -> Option<&UntypedItem<V>> { self.0.as_ref().map(|(_, item)| item) }

    /// Get the [`UntypedItem`] in the slot mutably.
    #[must_use]
    pub fn item_mut(&mut self) -> Option<&mut UntypedItem<V>> {
        self.0.as_mut().map(|(_, item)| item)
    }

    /// Set the [`UntypedItem`] in the slot.
    #[inline]
    pub fn set(&mut self, count: NonZeroU8, item: UntypedItem<V>) { self.0 = Some((count, item)); }

    /// Clear the slot.
    #[inline]
    pub fn clear(&mut self) { self.0 = None; }

    /// Consume one item from the slot.
    ///
    /// If there is only one item left, the slot will be emptied.
    pub fn consume(&mut self) {
        match self.count() {
            // Slot is already empty
            0 => (),
            // Clear the item from the slot
            1 => self.clear(),
            // Decrement the slot item count
            count => {
                if let (Some(count), Some(num)) = (self.count_mut(), NonZeroU8::new(count - 1)) {
                    *count = num;
                } else {
                    unreachable!("Count exists and is greater than 1");
                }
            }
        }
    }
}
