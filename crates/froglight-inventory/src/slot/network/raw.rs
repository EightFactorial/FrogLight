//! [`RawInventorySlot`], [`RawInventorySlotRef`], and networking
//! implementations.

use std::{borrow::Cow, marker::PhantomData, num::NonZeroU8};

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_common::prelude::*;
use froglight_io::{
    prelude::*,
    version::{FrogReadVersion, FrogWriteVersion},
};
use froglight_item::prelude::*;
use froglight_nbt::prelude::*;

use crate::slot::{
    InventorySlot,
    component::{InventoryComponents, VersionComponents},
};

/// An inventory slot with raw component data.
///
/// This is a raw, serialization-friendly form of [`InventorySlot`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
pub struct RawInventorySlot<V: Version>(
    pub(super) Option<(NonZeroU8, GlobalItemId, UnnamedNbt, Vec<u32>)>,
    pub(super) PhantomData<V>,
);

impl<V: VersionComponents> RawInventorySlot<V> {
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
    /// Returns `true` if the slot is empty.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.0.is_none() }

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
                // Get ids of any missing default components.
                let removed = Cow::Owned(Self::missing_defaults(item));

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
    fn missing_defaults(item: &UntypedItem<V>) -> Vec<u32> {
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
    #[inline]
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        RawInventorySlotRef::from_raw(self).frog_write(buffer)
    }

    #[inline]
    fn frog_len(&self) -> usize { RawInventorySlotRef::from_raw(self).frog_len() }
}

impl<V: Version> FrogWriteVersion<V> for RawInventorySlotRef<'_, V> {
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
                    for removed in removed.as_ref() {
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
                    for removed in removed.as_ref() {
                        length += u32::frog_var_len(removed);
                    }
                }

                length
            }
            None => u32::frog_var_len(&0),
        }
    }
}
