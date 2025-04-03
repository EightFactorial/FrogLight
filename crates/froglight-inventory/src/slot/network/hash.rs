use std::{borrow::Cow, marker::PhantomData, num::NonZeroU8};

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_common::prelude::Version;
use froglight_io::{
    prelude::*,
    version::{FrogReadVersion, FrogWriteVersion},
};
use froglight_item::{
    item::UntypedItem,
    storage::{GlobalItemId, ItemStorage},
};
use froglight_nbt::nbt::{NbtCompound, UnnamedNbt};

use crate::slot::{
    InventorySlot,
    component::{InventoryComponents, VersionComponents},
    network::RawInventorySlot,
};

/// An inventory slot with hashed component data.
///
/// This is a hashed, serialization-friendly form of [`InventorySlot`].
#[derive(Debug, Clone, PartialEq)]
#[expect(clippy::type_complexity)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
pub struct HashedInventorySlot<V: Version>(
    pub(super) Option<(NonZeroU8, GlobalItemId, UnnamedNbt, Vec<(u32, u32)>)>,
    pub(super) PhantomData<V>,
);

impl<V: VersionComponents> HashedInventorySlot<V> {
    /// Returns `true` if the slot is empty.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.0.is_none() }

    /// Create a [`RawInventorySlot`] from an [`InventorySlot`].
    ///
    /// Returns `None` if the item is not registered in the [`ItemStorage`].
    ///
    /// # Panics
    /// Panics if components are missing from [`InventoryComponents`].
    #[must_use]
    pub fn from_slot(slot: &InventorySlot<V>, storage: &ItemStorage<V>) -> Option<Self> {
        HashedInventorySlotRef::from_slot(slot, storage).map(HashedInventorySlotRef::into_hashed)
    }

    /// Create an [`InventorySlot`] from a [`RawInventorySlot`].
    ///
    /// Returns `None` if the item is not registered in the [`ItemStorage`].
    ///
    /// # Panics
    /// Panics if components are missing from [`InventoryComponents`].
    #[must_use]
    pub fn into_slot(self, storage: &ItemStorage<V>) -> Option<InventorySlot<V>> {
        HashedInventorySlotRef::from_hashed(&self).into_slot(storage)
    }

    /// Create a [`RawInventorySlot`] from a [`HashedInventorySlot`].
    ///
    /// # Note
    /// This skips the verification of the default component hashes.
    #[must_use]
    pub fn into_raw(self) -> RawInventorySlot<V> {
        HashedInventorySlotRef::from_hashed(&self).into_raw()
    }
}

// -------------------------------------------------------------------------------------------------

/// A reference to a [`HashedInventorySlot`].
///
/// This is a hashed, serialization-friendly form of [`InventorySlot`].
#[derive(Debug, Clone, PartialEq)]
#[expect(clippy::type_complexity)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
pub struct HashedInventorySlotRef<'a, V: Version>(
    Option<(NonZeroU8, GlobalItemId, &'a UnnamedNbt, Cow<'a, [(u32, u32)]>)>,
    PhantomData<V>,
);

impl<'a, V: Version> HashedInventorySlotRef<'a, V> {
    /// Returns `true` if the slot is empty.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.0.is_none() }

    /// Create a [`HashedInventorySlotRef`] to a [`HashedInventorySlot`].
    ///
    /// This is cheap and doesn't require any calculations.
    #[must_use]
    pub const fn from_hashed(slot: &'a HashedInventorySlot<V>) -> Self {
        match &slot.0 {
            Some((count, id, nbt, removed)) => {
                Self(Some((*count, *id, nbt, Cow::Borrowed(removed.as_slice()))), PhantomData)
            }
            None => Self(None, PhantomData),
        }
    }

    /// Create a [`HashedInventorySlot`] from an [`InventorySlot`].
    ///
    /// Returns `None` if the item is not registered in the [`ItemStorage`].
    #[must_use]
    pub fn from_slot(slot: &'a InventorySlot<V>, storage: &ItemStorage<V>) -> Option<Self>
    where V: VersionComponents {
        match &slot.0 {
            Some((count, item)) => {
                // Get the global item ID from the item.
                let global = storage.get_global(item)?;
                // Get ids of any missing default components.
                let removed = Cow::Owned(Self::missing_defaults(item));

                Some(Self(Some((*count, global, item.raw_data(), removed)), PhantomData))
            }
            None => Some(Self(None, PhantomData)),
        }
    }

    /// Create a [`HashedInventorySlot`] from a [`HashedInventorySlotRef`].
    #[must_use]
    pub fn into_hashed(self) -> HashedInventorySlot<V> {
        match self.0 {
            Some((count, global, nbt, removed)) => HashedInventorySlot(
                Some((count, global, nbt.clone(), removed.into_owned())),
                PhantomData,
            ),
            None => HashedInventorySlot(None, PhantomData),
        }
    }

    /// Create an [`InventorySlot`] from a [`HashedInventorySlotRef`].
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

    /// Create a [`RawInventorySlot`] from a [`HashedInventorySlotRef`].
    ///
    /// # Note
    /// This skips the verification of the default component hashes.
    #[must_use]
    pub fn into_raw(self) -> RawInventorySlot<V> {
        match self.0 {
            Some((count, global, nbt, removed)) => RawInventorySlot(
                Some((count, global, nbt.clone(), removed.iter().map(|(a, _)| *a).collect())),
                PhantomData,
            ),
            None => RawInventorySlot(None, PhantomData),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<V: Version> HashedInventorySlotRef<'_, V> {
    /// Get the ids and hashes of the default components that were removed.
    #[must_use]
    fn missing_defaults(item: &UntypedItem<V>) -> Vec<(u32, u32)> {
        let components = &*InventoryComponents::read::<V>();

        let mut removed = Vec::new();
        for (ident, data) in item.default_nbt().compound().into_iter().flatten() {
            let ident = ident.to_str_lossy();
            // If the item doesn't contain the component
            if !item.raw_data().contains_key(&ident) {
                match components.get_index_of(ident.as_ref()) {
                    // Add the removed component's index and hash
                    #[expect(clippy::cast_possible_truncation)]
                    Some(index) => {
                        let buf: Vec<u8> = data
                            .frog_to_buf()
                            .expect("Failed to serialize `InventoryComponent` NBT");

                        let hash = crc32fast::hash(&buf);
                        removed.push((index as u32, hash));
                    }
                    None => panic!("Unknown `InventoryComponent` \"{ident}\""),
                }
            }
        }
        removed
    }

    /// Remove a list of components from an item using their ids.
    fn remove_components(item: &mut UntypedItem<V>, removed: &[(u32, u32)]) {
        let components = &*InventoryComponents::read::<V>();

        if let Some(compound) = item.raw_data_mut().compound_mut() {
            for (id, _hash) in removed {
                // Get the component's identifier by index
                match components.get_index(*id as usize) {
                    // Remove the component from the item's NBT
                    Some((ident, _)) => {
                        if let Some(_data) = compound.swap_remove(ident.as_str()) {
                            // let buf: Vec<u8> = data
                            //     .frog_to_buf()
                            //     .expect("Failed to serialize
                            // `InventoryComponent` NBT");
                            //
                            // if hash != crc32fast::hash(&buf) {
                            //     warn!("Hash mismatch for
                            //       `InventoryComponent` \"{ident}\"");
                            // }
                        }
                    }
                    None => panic!("Unknown `InventoryComponent` ID: {id}"),
                }
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<V: Version> FrogReadVersion<V> for HashedInventorySlot<V> {
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
                    let id = u32::frog_var_read(buffer)?;
                    let hash = u32::frog_var_read(buffer)?;
                    removed.push((id, hash));
                }

                Ok(Self(Some((count, global, nbt, removed)), PhantomData))
            }
        }
    }
}

impl<V: Version> FrogWriteVersion<V> for HashedInventorySlot<V> {
    #[inline]
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        HashedInventorySlotRef::<V>::from_hashed(self).frog_write(buffer)
    }

    #[inline]
    fn frog_len(&self) -> usize { HashedInventorySlotRef::<V>::from_hashed(self).frog_len() }
}
impl<V: Version> FrogWriteVersion<V> for HashedInventorySlotRef<'_, V> {
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

                // Write the removed component ids and hashes.
                {
                    for (id, hash) in removed.as_ref() {
                        written += u32::frog_var_write(id, buffer)?;
                        written += u32::frog_var_write(hash, buffer)?;
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

                // Add the removed component id and hash lengths
                {
                    for (id, hash) in removed.as_ref() {
                        length += u32::frog_var_len(id);
                        length += u32::frog_var_len(hash);
                    }
                }

                length
            }
            None => u32::frog_var_len(&0),
        }
    }
}
