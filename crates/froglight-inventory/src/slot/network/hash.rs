use std::{borrow::Cow, marker::PhantomData, num::NonZeroU8};

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_common::prelude::Version;
use froglight_io::{
    prelude::*,
    version::{FrogReadVersion, FrogWriteVersion},
};
use froglight_item::storage::{GlobalItemId, ItemStorage};
use froglight_nbt::nbt::UnnamedNbt;

use super::RawInventorySlot;
use crate::slot::{InventorySlot, component::VersionComponents};

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
    pub fn from_slot(_slot: &'a InventorySlot<V>, _storage: &ItemStorage<V>) -> Option<Self>
    where V: VersionComponents {
        todo!()
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
    pub fn into_slot(self, _storage: &ItemStorage<V>) -> Option<InventorySlot<V>>
    where V: VersionComponents {
        todo!()
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

impl<V: Version> FrogReadVersion<V> for HashedInventorySlot<V> {
    fn frog_read(_buffer: &mut impl std::io::Read) -> Result<Self, ReadError> { todo!() }
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
    fn frog_write(&self, _buffer: &mut impl std::io::Write) -> Result<usize, WriteError> { todo!() }

    fn frog_len(&self) -> usize { todo!() }
}
