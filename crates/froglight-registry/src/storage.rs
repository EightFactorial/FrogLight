//! TODO

use alloc::{boxed::Box, sync::Arc};
use core::{any::TypeId, hash::Hash, marker::PhantomData};

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
use bevy_platform::hash::{FixedHasher, NoOpHash};
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::Deref;
use downcast_rs::Downcast;
use froglight_common::{identifier::Identifier, version::Version};
use hashbrown::HashMap;
use indexmap::{Equivalent, IndexMap};
use parking_lot::RwLock;

use crate::traits::{RegistryType, RegistryValue};

/// A thread-safe storage container for [`RegistryValue`]s.
#[derive(Clone, Deref)]
#[cfg_attr(feature = "bevy", derive(Reflect, Resource), reflect(Clone, Resource))]
pub struct AppRegistryStorage<V: Version>(Arc<RwLock<RegistryStorage<V>>>);

/// A storage container for [`RegistryValue`]s.
#[derive(Default)]
pub struct RegistryStorage<V: Version>(
    HashMap<TypeId, IndexMap<Identifier, Box<dyn RegistryValue>, FixedHasher>, NoOpHash>,
    PhantomData<V>,
);

impl<V: Version> RegistryStorage<V> {
    /// Create a new empty [`RegistryStorage`].
    #[inline]
    #[must_use]
    pub const fn new_empty() -> Self { Self(HashMap::with_hasher(NoOpHash), PhantomData) }

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
    pub fn keys(&self) -> impl Iterator<Item = &TypeId> { self.0.keys() }

    /// Get an iterator over all [`RegistryValue`]s of a specific
    /// [`RegistryType`].
    pub fn iter<R: RegistryType<V>>(&self) -> impl Iterator<Item = (&Identifier, &R::Value)> {
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
    ) -> impl Iterator<Item = (&Identifier, &mut R::Value)> {
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

    /// Insert the default values for a [`RegistryType`] into the
    /// [`RegistryStorage`].
    ///
    /// # Note
    /// This will overwrite any existing values.
    pub fn insert_default<R: RegistryType<V>>(&mut self) {
        self.extend::<R, _>(<R as RegistryType<V>>::defaults());
    }

    /// Extend the [`RegistryStorage`] with a collection of [`RegistryValue`]s.
    ///
    /// This is the same as calling [`RegistryStorage::insert`] for each item.
    ///
    /// See [`IndexMap::extend`] for more information.
    pub fn extend<R: RegistryType<V>, I: IntoIterator<Item = (Identifier, R::Value)>>(
        &mut self,
        iter: I,
    ) {
        self.0.entry(TypeId::of::<R>()).or_default().extend(iter.into_iter().map(
            |(ident, value)| -> (Identifier, Box<dyn RegistryValue>) { (ident, Box::new(value)) },
        ));
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

    /// Remove all [`RegistryValue`]s of all [`RegistryType`]s.
    ///
    /// Keeps the allocated memory for future use.
    #[inline]
    pub fn clear_all(&mut self) { self.0.clear(); }
}
