//! TODO

use std::{any::TypeId, hash::Hash, marker::PhantomData, sync::Arc};

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
#[cfg(feature = "bevy")]
use bevy_utils::TypeIdMap;
use downcast_rs::Downcast;
use froglight_common::{Identifier, Version};
use hashbrown::Equivalent;
use indexmap::IndexMap;
use parking_lot::RwLock;

use crate::traits::{RegistryType, RegistryValue};

/// A thread-safe storage container for [`RegistryValue`]s.
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Reflect, Resource), reflect(Resource))]
pub struct AppRegistryStorage<V: Version>(Arc<RwLock<RegistryStorage<V>>>);

/// A storage container for [`RegistryValue`]s.
#[derive(Default)]
pub struct RegistryStorage<V: Version>(
    #[cfg(feature = "bevy")] TypeIdMap<IndexMap<Identifier, Box<dyn RegistryValue>>>,
    #[cfg(not(feature = "bevy"))] HashMap<TypeId, IndexMap<Identifier, Box<dyn RegistryValue>>>,
    PhantomData<V>,
);

impl<V: Version> RegistryStorage<V> {
    /// Create a new empty [`RegistryStorage`].
    #[inline]
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Get a [`RegistryValue`] by it's [`RegistryType`] and identifier.
    ///
    /// Returns `None` if the value does not exist.
    #[must_use]
    pub fn get<R: RegistryType<V>>(
        &self,
        ident: &(impl Equivalent<Identifier> + Hash + ?Sized),
    ) -> Option<&R::Value> {
        self.0
            .get(&TypeId::of::<R>())
            .and_then(|map| map.get(ident))
            .and_then(|v| <dyn RegistryValue as Downcast>::as_any(v.as_ref()).downcast_ref())
    }

    /// Get a [`RegistryValue`] by it's [`RegistryType`] and index
    ///
    /// Returns `None` if the value does not exist.
    #[must_use]
    pub fn get_index<R: RegistryType<V>>(&self, index: usize) -> Option<(&Identifier, &R::Value)> {
        self.0.get(&TypeId::of::<R>()).and_then(|map| map.get_index(index)).and_then(|(i, v)| {
            <dyn RegistryValue as Downcast>::as_any(v.as_ref()).downcast_ref().map(|v| (i, v))
        })
    }

    /// Get a mutable [`RegistryValue`] by it's [`RegistryType`] and identifier.
    ///
    /// Returns `None` if the value does not exist.
    #[must_use]
    pub fn get_mut<R: RegistryType<V>>(
        &mut self,
        ident: &(impl Equivalent<Identifier> + Hash + ?Sized),
    ) -> Option<&mut R::Value> {
        self.0
            .get_mut(&TypeId::of::<R>())
            .and_then(|map| map.get_mut(ident))
            .and_then(|v| <dyn RegistryValue as Downcast>::as_any_mut(v.as_mut()).downcast_mut())
    }

    /// Get a mutable [`RegistryValue`] by it's [`RegistryType`] and index.
    ///
    /// Returns `None` if the value does not exist.
    #[must_use]
    pub fn get_index_mut<R: RegistryType<V>>(
        &mut self,
        index: usize,
    ) -> Option<(&Identifier, &mut R::Value)> {
        self.0.get_mut(&TypeId::of::<R>()).and_then(|map| map.get_index_mut(index)).and_then(
            |(i, v)| {
                <dyn RegistryValue as Downcast>::as_any_mut(v.as_mut())
                    .downcast_mut()
                    .map(|v| (i, v))
            },
        )
    }

    /// Returns `true` if the [`RegistryStorage`] contains
    /// a [`RegistryValue`] with the given identifier.
    #[must_use]
    pub fn contains<R: RegistryType<V>>(
        &self,
        ident: &(impl Equivalent<Identifier> + Hash + ?Sized),
    ) -> bool {
        self.0.get(&TypeId::of::<R>()).is_some_and(|map| map.contains_key(ident))
    }

    /// Iterate over all [`RegistryType`]s in the [`RegistryStorage`]
    /// in an arbitrary order.
    pub fn keys(&self) -> impl Iterator<Item = &TypeId> + '_ { self.0.keys() }

    /// Get an iterator over all [`RegistryValue`]s of a specific
    /// [`RegistryType`].
    pub fn iter<R: RegistryType<V>>(&self) -> impl Iterator<Item = (&Identifier, &R::Value)> + '_ {
        self.0.get(&TypeId::of::<R>()).into_iter().flat_map(|map| {
            map.iter().filter_map(|(k, v)| {
                <dyn RegistryValue as Downcast>::as_any(v.as_ref()).downcast_ref().map(|v| (k, v))
            })
        })
    }

    /// Get a mutable iterator over all [`RegistryValue`]s of a specific
    /// [`RegistryType`].
    pub fn iter_mut<R: RegistryType<V>>(
        &mut self,
    ) -> impl Iterator<Item = (&Identifier, &mut R::Value)> + '_ {
        self.0.entry(TypeId::of::<R>()).or_default().iter_mut().filter_map(|(k, v)| {
            <dyn RegistryValue as Downcast>::as_any_mut(v.as_mut()).downcast_mut().map(|v| (k, v))
        })
    }

    /// Insert a [`RegistryValue`] into the [`RegistryStorage`].
    ///
    /// Returns the previous value if it existed.
    pub fn insert<R: RegistryType<V>>(
        &mut self,
        ident: Identifier,
        value: R::Value,
    ) -> Option<R::Value> {
        self.0.entry(TypeId::of::<R>()).or_default().insert(ident, Box::new(value)).and_then(|v| {
            <dyn RegistryValue as Downcast>::into_any(v).downcast().map_or(None, |v| Some(*v))
        })
    }

    /// Insert a [`RegistryValue`] into the [`RegistryStorage`] at a specific
    /// index, shifting all other values to the right.
    ///
    /// Returns the previous value if it existed.
    ///
    /// See [`IndexMap::shift_insert`] for more information.
    pub fn shift_insert<R: RegistryType<V>>(
        &mut self,
        index: usize,
        ident: Identifier,
        value: R::Value,
    ) -> Option<R::Value> {
        self.0
            .entry(TypeId::of::<R>())
            .or_default()
            .shift_insert(index, ident, Box::new(value))
            .and_then(|v| {
                <dyn RegistryValue as Downcast>::into_any(v).downcast().map_or(None, |v| Some(*v))
            })
    }

    /// Remove a [`RegistryValue`] from the [`RegistryStorage`].
    ///
    /// Returns `None` if the value does not exist.
    ///
    /// See [`IndexMap::swap_remove`] for more information.
    #[must_use]
    pub fn swap_remove<R: RegistryType<V>>(
        &mut self,
        ident: &(impl Equivalent<Identifier> + Hash + ?Sized),
    ) -> Option<R::Value> {
        self.0.get_mut(&TypeId::of::<R>()).and_then(|map| map.swap_remove(ident)).and_then(|v| {
            <dyn RegistryValue as Downcast>::into_any(v).downcast().map_or(None, |v| Some(*v))
        })
    }

    /// Remove a [`RegistryValue`] from the [`RegistryStorage`].
    ///
    /// Returns `None` if the value does not exist.
    ///
    /// See [`IndexMap::shift_remove`] for more information.
    pub fn shift_remove<R: RegistryType<V>>(
        &mut self,
        ident: &(impl Equivalent<Identifier> + Hash + ?Sized),
    ) -> Option<R::Value> {
        self.0.get_mut(&TypeId::of::<R>()).and_then(|map| map.shift_remove(ident)).and_then(|v| {
            <dyn RegistryValue as Downcast>::into_any(v).downcast().map_or(None, |v| Some(*v))
        })
    }

    /// Remove all [`RegistryValue`]s of a specific [`RegistryType`].
    ///
    /// Keeps the allocated memory for future use.
    pub fn clear<R: RegistryType<V>>(&mut self) {
        self.0.get_mut(&TypeId::of::<R>()).map(IndexMap::clear);
    }
}

impl<V: Version> RegistryStorage<V> {
    /// Get a [`RegistryValue`] as [`BaseNbt`]
    /// by it's [`RegistryType`] and identifier.
    ///
    /// ### Note
    /// If the value does not implement [`Copy`],
    /// use [`RegistryStorage::get_cloned_nbt`] instead.
    #[inline]
    pub fn get_copied_nbt<R: RegistryType<V>>(
        &self,
        ident: &(impl Equivalent<Identifier> + Hash + ?Sized),
    ) -> Option<simdnbt::owned::BaseNbt>
    where
        R::Value: Copy + simdnbt::Serialize,
    {
        self.get::<R>(ident).copied().map(<R::Value as simdnbt::Serialize>::to_nbt)
    }

    /// Get a [`RegistryValue`] as [`BaseNbt`]
    /// by it's [`RegistryType`] and identifier.
    ///
    /// ### Note
    /// If the value implements [`Copy`],
    /// prefer [`RegistryStorage::get_copied_nbt`] instead.
    #[inline]
    pub fn get_cloned_nbt<R: RegistryType<V>>(
        &self,
        ident: &(impl Equivalent<Identifier> + Hash + ?Sized),
    ) -> Option<simdnbt::owned::BaseNbt>
    where
        R::Value: Clone + simdnbt::Serialize,
    {
        self.get::<R>(ident).cloned().map(<R::Value as simdnbt::Serialize>::to_nbt)
    }

    /// Insert a serialized [`RegistryValue`] into the [`RegistryStorage`].
    ///
    /// Returns the previous value if it existed.
    ///
    /// # Errors
    /// Returns an error if the value could not be deserialized.
    #[inline]
    pub fn insert_nbt<R: RegistryType<V>>(
        &mut self,
        ident: Identifier,
        nbt: &simdnbt::borrow::BaseNbt,
    ) -> Result<Option<R::Value>, simdnbt::DeserializeError>
    where
        R::Value: simdnbt::Deserialize,
    {
        <R::Value as simdnbt::Deserialize>::from_nbt(nbt)
            .map(|value: R::Value| self.insert::<R>(ident, value))
    }
}
