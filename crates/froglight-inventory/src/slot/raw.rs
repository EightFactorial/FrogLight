use std::num::NonZeroU8;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_io::prelude::*;
use froglight_item::storage::GlobalItemId;
use froglight_nbt::nbt::{NbtCompound, UnnamedNbt};

use super::{InventoryComponents, InventorySlot, component::VersionComponents};

/// A raw inventory slot.
///
/// This is a raw, serialization-friendly form of [`InventorySlot`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
pub struct RawInventorySlot(Option<(NonZeroU8, GlobalItemId, UnnamedNbt, Vec<u32>)>);

impl RawInventorySlot {
    /// Create a [`RawInventorySlot`] from an [`InventorySlot`].
    ///
    /// Returns `None` if the item is not registered in the [`ItemStorage`].
    ///
    /// # Panics
    /// Panics if components are missing from [`InventoryComponents`].
    #[must_use]
    pub fn from_slot<V: VersionComponents>(
        slot: InventorySlot<V>,
        storage: &froglight_item::storage::ItemStorage<V>,
    ) -> Option<Self> {
        match slot.0 {
            Some((count, item)) => {
                // Get the global item ID.
                let global = storage.get_global(&item)?;

                // Create a list of removed defaults.
                let mut removed = Vec::new();
                {
                    let components = &*InventoryComponents::read::<V>();
                    for (default, _) in item.default_nbt().iter().flat_map(|c| c.iter()) {
                        let default = default.to_str_lossy();
                        // If the item does not contain a default component
                        if !item.raw_data().contains_key(&default) {
                            match components.get_index_of(default.as_ref()) {
                                // Push the index of the component to the `removed` list.
                                #[expect(clippy::cast_possible_truncation)]
                                Some(component) => removed.push(component as u32),
                                // Panic, the component is missing from `InventoryComponents`.
                                None => panic!("Missing `InventoryComponent` for `{default}`"),
                            }
                        }
                    }
                }

                // Return the raw inventory slot.
                Some(Self(Some((count, global, item.into_inner(), removed))))
            }
            None => Some(Self(None)),
        }
    }

    /// Create an [`InventorySlot`] from a [`RawInventorySlot`].
    ///
    /// Returns `None` if the item is not registered in the [`ItemStorage`].
    ///
    /// # Panics
    /// Panics if components are missing from [`InventoryComponents`].
    #[must_use]
    #[expect(clippy::needless_for_each)]
    pub fn into_slot<V: VersionComponents>(
        self,
        storage: &froglight_item::storage::ItemStorage<V>,
    ) -> Option<InventorySlot<V>> {
        match self.0 {
            Some((count, item, data, removed)) => {
                // Get the item from the global item ID.
                let mut item = storage.get_untyped(item, Some(data))?;

                {
                    // Get the defaults and remove the component specified in `removed`.
                    let mut defaults = item.default_nbt();
                    let components = &*InventoryComponents::read::<V>();
                    removed.into_iter().for_each(|index| {
                        match components.get_index(index as usize) {
                            // Remove the default component.
                            Some((component, _)) => {
                                defaults.compound_mut().unwrap().swap_remove(component);
                            }
                            // Panic, the component is missing from `InventoryComponents`.
                            None => panic!("Missing `InventoryComponent` for index `{index}`"),
                        }
                    });

                    // Insert the default components that are not already present.
                    for (default, value) in
                        defaults.into_inner().into_iter().flat_map(NbtCompound::into_iter)
                    {
                        if !item.raw_data().contains_key_bytes(default.as_bytes()) {
                            item.raw_data_mut().insert(default, value);
                        }
                    }
                }

                // Return the inventory slot.
                Some(InventorySlot::new_from(count, item))
            }
            None => Some(InventorySlot::new()),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl FrogRead for RawInventorySlot {
    fn frog_read(_buffer: &mut impl std::io::Read) -> Result<Self, ReadError> { todo!() }
}

impl FrogWrite for RawInventorySlot {
    fn frog_write(&self, _buffer: &mut impl std::io::Write) -> Result<usize, WriteError> { todo!() }

    fn frog_len(&self) -> usize { todo!() }
}
