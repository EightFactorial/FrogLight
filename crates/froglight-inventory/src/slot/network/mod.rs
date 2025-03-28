//! [`RawInventorySlot`], [`RawInventorySlotRef`], and networking
//! implementations.

use std::{borrow::Cow, marker::PhantomData, num::NonZeroU8};

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_common::prelude::Version;
use froglight_item::{
    item::UntypedItem,
    storage::{GlobalItemId, ItemStorage},
};
use froglight_nbt::nbt::UnnamedNbt;

use super::{
    InventorySlot,
    component::{InventoryComponents, VersionComponents},
};

mod other;
mod v1_21_4;

/// A raw inventory slot.
///
/// This is a raw, serialization-friendly form of [`InventorySlot`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
pub struct RawInventorySlot<V: Version>(
    Option<(NonZeroU8, GlobalItemId, UnnamedNbt, Vec<u32>)>,
    PhantomData<V>,
);

impl<V: VersionComponents> RawInventorySlot<V> {
    /// Create a [`RawInventorySlot`] from an [`InventorySlot`].
    ///
    /// Returns `None` if the item is not registered in the [`ItemStorage`].
    ///
    /// # Panics
    /// Panics if components are missing from [`InventoryComponents`].
    #[must_use]
    pub fn from_slot(slot: &InventorySlot<V>, storage: &ItemStorage<V>) -> Option<Self> {
        RawInventorySlotRef::from_slot(slot, storage).map(RawInventorySlotRef::into_raw)
    }

    /// Create an [`InventorySlot`] from a [`RawInventorySlot`].
    ///
    /// Returns `None` if the item is not registered in the [`ItemStorage`].
    ///
    /// # Panics
    /// Panics if components are missing from [`InventoryComponents`].
    #[must_use]
    pub fn into_slot(self, storage: &ItemStorage<V>) -> Option<InventorySlot<V>> {
        RawInventorySlotRef::from_raw(&self).into_slot(storage)
    }
}

// -------------------------------------------------------------------------------------------------

/// A reference to a [`RawInventorySlot`].
///
/// This is a raw, serialization-friendly form of [`InventorySlot`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
pub struct RawInventorySlotRef<'a, V: Version>(
    Option<(NonZeroU8, GlobalItemId, &'a UnnamedNbt, Cow<'a, [u32]>)>,
    PhantomData<V>,
);

impl<'a, V: Version> RawInventorySlotRef<'a, V> {
    /// Create a [`RawInventorySlotRef`] to a [`RawInventorySlot`].
    ///
    /// This is cheap and doesn't require any calculations.
    #[must_use]
    pub const fn from_raw(slot: &'a RawInventorySlot<V>) -> Self {
        match &slot.0 {
            Some((count, global, nbt, removed)) => {
                Self(Some((*count, *global, nbt, Cow::Borrowed(removed.as_slice()))), PhantomData)
            }
            None => Self(None, PhantomData),
        }
    }

    /// Create a [`RawInventorySlotRef`] to an [`InventorySlot`].
    ///
    /// Returns `None` if the item is not registered in the [`ItemStorage`].
    #[must_use]
    pub fn from_slot(slot: &'a InventorySlot<V>, storage: &ItemStorage<V>) -> Option<Self>
    where V: VersionComponents {
        match &slot.0 {
            Some((count, item)) => {
                // Get the global item ID from the item.
                let global = storage.get_global(item)?;
                // Get the removed default components.
                let removed = Cow::Owned(Self::removed_defaults(item));

                Some(Self(Some((*count, global, item.raw_data(), removed)), PhantomData))
            }
            None => Some(Self(None, PhantomData)),
        }
    }

    /// Create an [`RawInventorySlot`] from a [`RawInventorySlotRef`].
    #[must_use]
    pub fn into_raw(self) -> RawInventorySlot<V> {
        match self.0 {
            Some((count, global, nbt, removed)) => RawInventorySlot(
                Some((count, global, nbt.clone(), removed.into_owned())),
                PhantomData,
            ),
            None => RawInventorySlot(None, PhantomData),
        }
    }

    /// Create an [`InventorySlot`] from a [`RawInventorySlotRef`].
    ///
    /// Returns `None` if the item is not registered in the [`ItemStorage`].
    #[must_use]
    pub fn into_slot(self, storage: &ItemStorage<V>) -> Option<InventorySlot<V>>
    where V: VersionComponents {
        match self.0 {
            Some((count, item, nbt, removed)) => {
                // Get the item from the global item ID.
                let mut item = storage.get_untyped(item, None)?;
                // Remove the removed default components.
                Self::remove_components(&mut item, removed.as_ref());

                // Overwrite the item's NBT with the slot's NBT.
                if let Some(compound) = nbt.clone().into_inner() {
                    for (ident, tag) in compound {
                        item.raw_data_mut().insert(ident, tag);
                    }
                }

                Some(InventorySlot::new_from(count, item))
            }
            None => Some(InventorySlot::new_empty()),
        }
    }
}

impl<V: Version> RawInventorySlotRef<'_, V> {
    /// Get the ids of the default components that were removed.
    #[must_use]
    fn removed_defaults(item: &UntypedItem<V>) -> Vec<u32> {
        let components = &*InventoryComponents::read::<V>();

        let mut removed = Vec::new();
        for (ident, _) in item.default_nbt().compound().into_iter().flatten() {
            let ident = ident.to_str_lossy();
            // If the item doesn't contain the component
            if !item.raw_data().contains_key(&ident) {
                match components.get_index_of(ident.as_ref()) {
                    // Add the removed component's index
                    #[expect(clippy::cast_possible_truncation)]
                    Some(index) => removed.push(index as u32),
                    None => panic!("Unknown `InventoryComponent` \"{ident}\""),
                }
            }
        }
        removed
    }

    /// Remove a list of components from an item using their ids.
    fn remove_components(item: &mut UntypedItem<V>, removed: &[u32]) {
        let components = &*InventoryComponents::read::<V>();

        if let Some(compound) = item.raw_data_mut().compound_mut() {
            for &removed in removed {
                // Get the component's identifier by index
                match components.get_index(removed as usize) {
                    // Remove the component from the item's NBT
                    Some((ident, _)) => compound.swap_remove(ident.as_str()),
                    None => panic!("Unknown `InventoryComponent` ID: {removed}"),
                };
            }
        }
    }
}
