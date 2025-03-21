use std::{marker::PhantomData, num::NonZeroU8};

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_common::prelude::Version;
use froglight_io::{
    prelude::*,
    version::{FrogReadVersion, FrogWriteVersion},
};
use froglight_item::storage::{GlobalItemId, ItemStorage};
use froglight_nbt::nbt::{NbtCompound, UnnamedNbt};

use super::{
    InventorySlot,
    component::{InventoryComponents, VersionComponents},
};

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
        match &slot.0 {
            Some((count, item)) => {
                // Get the global item ID from the item.
                let global = storage.get_global(item)?;

                // Get the removed default components.
                let mut removed = Vec::new();
                {
                    let components = &*InventoryComponents::read::<V>();
                    for (ident, _) in item.default_nbt().compound().into_iter().flatten() {
                        let ident = ident.to_str_lossy();
                        if !item.raw_data().contains_key(&ident) {
                            match components.get_index_of(ident.as_ref()) {
                                #[expect(clippy::cast_possible_truncation)]
                                Some(index) => removed.push(index as u32),
                                None => panic!("Unknown `InventoryComponent` \"{ident}\""),
                            }
                        }
                    }
                }

                Some(Self(Some((*count, global, item.raw_data().clone(), removed)), PhantomData))
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
            Some((count, item, nbt, removed)) => {
                // Get the item from the global item ID.
                let mut item = storage.get_untyped(item, None)?;

                // Remove the removed default components.
                {
                    let components = &*InventoryComponents::read::<V>();
                    if let Some(compound) = item.raw_data_mut().compound_mut() {
                        for removed in removed {
                            // Get the component's identifier by index.
                            match components.get_index(removed as usize) {
                                // Remove the component from the item's NBT.
                                Some((ident, _)) => compound.swap_remove(ident.as_str()),
                                None => panic!("Unknown `InventoryComponent` ID: {removed}"),
                            };
                        }
                    }
                }

                // Overwrite the item's NBT with the slot's NBT.
                {
                    if let Some(compound) = nbt.into_inner() {
                        for (ident, tag) in compound {
                            item.raw_data_mut().insert(ident, tag);
                        }
                    }
                }

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
                let count = NonZeroU8::new(count as u8).unwrap();
                let global = GlobalItemId::new_unchecked(u32::frog_var_read(buffer)?);

                let add_len = u32::frog_var_read(buffer)? as usize;
                let rem_len = u32::frog_var_read(buffer)? as usize;

                let mut nbt = UnnamedNbt::new_empty();

                // Read the added component data.
                {
                    let components = &*InventoryComponents::read::<V>();
                    for _ in 0..add_len {
                        let index = u32::frog_var_read(buffer)? as usize;
                        // Get the component's reader by index.
                        match components.get_index(index) {
                            None => panic!("Unknown `InventoryComponent` ID: {index}"),
                            Some((ident, fns)) => {
                                // Read the component data and insert it into `nbt`.
                                match fns.frog_read(buffer) {
                                    Ok(tag) => nbt.insert(ident.as_str(), tag),
                                    Err(err) => {
                                        panic!(
                                            "Failed to read `InventoryComponent` \"{ident}\": {err}"
                                        )
                                    }
                                };
                            }
                        }
                    }
                }

                // Read the removed component ids.
                let mut removed = Vec::with_capacity(rem_len);
                for _ in 0..rem_len {
                    removed.push(u32::frog_var_read(buffer)?);
                }

                Ok(Self(Some((count, global, nbt, removed)), PhantomData))
            }
        }
    }
}

impl<V: Version> FrogWriteVersion<V> for RawInventorySlot<V> {
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        match &self.0 {
            Some((count, global, data, removed)) => {
                let mut written = 0;

                written += u32::from(count.get()).frog_var_write(buffer)?;
                written += u32::from(*global).frog_var_write(buffer)?;

                #[expect(clippy::cast_possible_truncation)]
                {
                    let count = data.compound().map_or(0, NbtCompound::len);
                    written += u32::frog_var_write(&(count as u32), buffer)?;
                    written += u32::frog_var_write(&(removed.len() as u32), buffer)?;
                }

                // Write the added component data.
                {
                    let components = &*InventoryComponents::read::<V>();
                    for (ident, tag) in data.compound().into_iter().flatten() {
                        let ident = ident.to_str_lossy();
                        // Get the component's index by identifier.
                        match components.get_full(ident.as_ref()) {
                            None => panic!("Unknown `InventoryComponent` \"{ident}\""),
                            #[expect(clippy::cast_possible_truncation)]
                            Some((index, _, fns)) => {
                                written += u32::frog_var_write(&(index as u32), buffer)?;
                                written += fns.frog_write(tag, buffer)?;
                            }
                        }
                    }
                }

                // Write the removed component ids.
                {
                    for removed in removed {
                        written += u32::frog_var_write(removed, buffer)?;
                    }
                }

                Ok(written)
            }
            None => u32::frog_var_write(&0, buffer),
        }
    }

    fn frog_len(&self) -> usize {
        match &self.0 {
            Some((count, global, data, removed)) => {
                let mut length = 0;

                length += u32::from(count.get()).frog_var_len();
                length += u32::from(*global).frog_var_len();

                #[expect(clippy::cast_possible_truncation)]
                {
                    let count = data.compound().map_or(0, NbtCompound::len);
                    length += u32::frog_var_len(&(count as u32));
                    length += u32::frog_var_len(&(removed.len() as u32));
                }

                // Add the added component data lengths
                {
                    let components = &*InventoryComponents::read::<V>();
                    for (ident, tag) in data.compound().into_iter().flatten() {
                        let ident = ident.to_str_lossy();
                        // Get the component's index by identifier.
                        match components.get_full(ident.as_ref()) {
                            None => panic!("Unknown `InventoryComponent` \"{ident}\""),
                            #[expect(clippy::cast_possible_truncation)]
                            Some((index, _, fns)) => {
                                length += u32::frog_var_len(&(index as u32));
                                length += fns.frog_len(tag);
                            }
                        }
                    }
                }

                // Add the removed component ids lengths
                {
                    for removed in removed {
                        length += u32::frog_var_len(removed);
                    }
                }

                length
            }
            None => u32::frog_var_len(&0),
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[test]
#[cfg(test)]
#[cfg(feature = "v1_21_4")]
fn empty() {
    use std::io::Cursor;

    use froglight_common::version::V1_21_4;

    let slot = InventorySlot::<V1_21_4>::new_empty();
    let storage = ItemStorage::<V1_21_4>::new();

    let raw = RawInventorySlot::from_slot(&slot, &storage).unwrap();

    let mut buffer = Vec::new();
    raw.frog_write(&mut buffer).unwrap();
    assert_eq!(raw.frog_len(), buffer.len());

    assert_eq!(RawInventorySlot::<V1_21_4>::frog_read(&mut Cursor::new(buffer)).unwrap(), raw)
}

// #[test]
// #[cfg(test)]
// #[cfg(feature = "v1_21_4")]
// fn full() {
//     use std::io::Cursor;
//
//     use froglight_common::version::V1_21_4;
//     use froglight_item::prelude::*;
//
//     let storage = ItemStorage::<V1_21_4>::new();
//
//     let item = Item::<item::Anvil, V1_21_4>::default();
//     let slot = InventorySlot::<V1_21_4>::new_from(NonZeroU8::new(8).unwrap(),
// item.into_untyped());
//
//     let raw = RawInventorySlot::from_slot(&slot, &storage).unwrap();
//
//     let mut buffer = Vec::new();
//     raw.frog_write(&mut buffer).unwrap();
//     assert_eq!(raw.frog_len(), buffer.len());
//
//     assert_eq!(RawInventorySlot::<V1_21_4>::frog_read(&mut
// Cursor::new(buffer)).unwrap(), raw) }
