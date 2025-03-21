use std::{io::Cursor, marker::PhantomData, num::NonZeroU8};

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_common::prelude::Version;
use froglight_io::{
    prelude::*,
    version::{FrogReadVersion, FrogWriteVersion},
};
use froglight_item::storage::{GlobalItemId, ItemStorage};

use super::{InventoryComponents, InventorySlot, component::VersionComponents};

/// A raw inventory slot.
///
/// This is a raw, serialization-friendly form of [`InventorySlot`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
pub struct RawInventorySlot<V: Version>(
    Option<(NonZeroU8, GlobalItemId, Vec<u8>, Vec<u32>)>,
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
    pub fn from_slot(slot: InventorySlot<V>, storage: &ItemStorage<V>) -> Option<Self> {
        match slot.0 {
            Some((_count, item)) => {
                // Get the global item ID.
                let _global = storage.get_global(&item)?;

                todo!()
            }
            None => Some(Self(None, PhantomData)),
        }
    }

    /// Create an [`InventorySlot`] from a [`RawInventorySlot`].
    ///
    /// Returns `None` if the item is not registered in the [`ItemStorage`].
    ///
    /// # Panics
    /// Panics if components are missing from [`InventoryComponents`].
    #[must_use]
    pub fn into_slot(self, storage: &ItemStorage<V>) -> Option<InventorySlot<V>> {
        match self.0 {
            Some((count, item, data, removed)) => {
                // Get the item from the global item ID.
                let mut item = storage.get_untyped(item, None)?;
                let mut nbt = item.default_nbt();

                let components = &*InventoryComponents::read::<V>();

                // Read the added component data.
                {
                    let mut cursor = Cursor::new(data);
                    while cursor.position() < cursor.get_ref().len() as u64 {
                        let index = u32::frog_var_read(&mut cursor);
                        let index = index.expect("Failed to read `InventoryComponent` ID");

                        match components
                            .get_index(index as usize)
                            .map(|(ident, fns)| (ident, fns.frog_read(&mut cursor)))
                        {
                            // Successfully read components, add to NBT.
                            Some((ident, Ok(tag))) => {
                                nbt.insert(ident.as_str(), tag);
                            }
                            // Failed, panic.
                            Some((ident, Err(err))) => {
                                panic!("Failed to read `InventoryComponent` \"{ident}\": {err}")
                            }
                            None => panic!("Unknown `InventoryComponent` ID: {index}"),
                        }
                    }
                }

                // Read the removed component ids.
                {
                    if let Some(compound) = nbt.compound_mut() {
                        for removed in removed {
                            match components.get_index(removed as usize) {
                                Some((ident, _)) => compound.swap_remove(ident.as_str()),
                                None => panic!("Unknown `InventoryComponent` ID: {removed}"),
                            };
                        }
                    }
                }

                // Return the inventory slot.
                *item.raw_data_mut() = nbt;
                Some(InventorySlot::new_from(count, item))
            }
            None => Some(InventorySlot::new_empty()),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<V: Version> FrogReadVersion<V> for RawInventorySlot<V> {
    fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, ReadError> {
        match u32::frog_var_read(buffer)? {
            0 => Ok(Self(None, PhantomData)),
            count => {
                #[expect(clippy::cast_possible_truncation)]
                let _count = NonZeroU8::new(count as u8).unwrap();
                let _global = u32::frog_var_read(buffer)?;
                let _add_len = u32::frog_var_read(buffer)?;
                let _rem_len = u32::frog_var_read(buffer)?;

                todo!()
            }
        }
    }
}

impl<V: Version> FrogWriteVersion<V> for RawInventorySlot<V> {
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        match &self.0 {
            Some((_count, _global, _data, _removed)) => todo!(),
            None => u32::frog_var_write(&0, buffer),
        }
    }

    fn frog_len(&self) -> usize {
        match &self.0 {
            Some((_count, _global, _data, _removed)) => todo!(),
            None => u32::frog_var_len(&0),
        }
    }
}
