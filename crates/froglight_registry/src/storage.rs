//! TODO

use core::{fmt::Debug, hash::Hash, ops::Deref};

use foldhash::fast::RandomState;
use froglight_common::{prelude::Identifier, version::Version};
use indexmap::{Equivalent, IndexMap, IndexSet};

/// A [`Version`] with an associated [`RegistryMap`].
pub trait Registry: Version {
    /// Get the [`StaticRegistryMap`] for this [`Version`].
    fn registry() -> &'static StaticRegistryMap;
    /// Initialize this version's registries into the provided [`RegistryMap`].
    fn init_registry(map: &mut RegistryMap);
}

// -------------------------------------------------------------------------------------------------

/// A modifiable, thread-safe reference to a [`RegistryMap`].
pub struct StaticRegistryMap {
    #[cfg(feature = "async")]
    map: async_lock::RwLock<RegistryMap>,
    #[cfg(not(feature = "async"))]
    map: parking_lot::RwLock<RegistryMap>,
    /// A function to reset the [`RegistryMap`] to its initial state.
    reset: fn(&mut RegistryMap),
}

impl StaticRegistryMap {
    /// Create a new [`StaticRegistryMap`].
    #[must_use]
    #[cfg(feature = "async")]
    pub const fn new<V: Registry>(map: RegistryMap) -> Self {
        StaticRegistryMap {
            map: async_lock::RwLock::new(map),
            reset: |map: &mut RegistryMap| {
                map.clear();
                V::init_registry(map);
            },
        }
    }

    /// Read the [`RegistryMap`], blocking the current thread if necessary.
    #[must_use]
    #[cfg(feature = "async")]
    pub fn read_blocking(&self) -> async_lock::RwLockReadGuard<'_, RegistryMap> {
        self.map.read_blocking()
    }

    /// Write to the [`RegistryMap`], blocking the current thread if necessary.
    #[must_use]
    #[cfg(feature = "async")]
    pub fn write_blocking(&self) -> async_lock::RwLockWriteGuard<'_, RegistryMap> {
        self.map.write_blocking()
    }

    /// Create a new [`StaticRegistryMap`].
    #[must_use]
    #[cfg(not(feature = "async"))]
    pub const fn new<V: Registry>(map: RegistryMap) -> Self {
        StaticRegistryMap {
            map: parking_lot::RwLock::new(map),
            reset: |map: &mut RegistryMap| {
                map.clear();
                V::init_registry(map);
            },
        }
    }

    /// Read the [`RegistryMap`], blocking the current thread if necessary.
    #[cfg(not(feature = "async"))]
    pub fn read_blocking(&self) -> parking_lot::RwLockReadGuard<'_, RegistryMap> { self.map.read() }

    /// Write to the [`RegistryMap`], blocking the current thread if necessary.
    #[cfg(not(feature = "async"))]
    pub fn write_blocking(&self) -> parking_lot::RwLockWriteGuard<'_, RegistryMap> {
        self.map.write()
    }

    /// Reset the [`RegistryMap`] to its initial state.
    pub fn reset(&self) { (self.reset)(&mut self.write_blocking()); }
}

impl Debug for StaticRegistryMap {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StaticRegistryMap").finish_non_exhaustive()
    }
}

impl Deref for StaticRegistryMap {
    #[cfg(feature = "async")]
    type Target = async_lock::RwLock<RegistryMap>;
    #[cfg(not(feature = "async"))]
    type Target = parking_lot::RwLock<RegistryMap>;

    fn deref(&self) -> &Self::Target { &self.map }
}

// -------------------------------------------------------------------------------------------------

/// A map of registry [`Identifier`]s to their corresponding [`RegistrySet`]s.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RegistryMap(IndexMap<Identifier, RegistrySet, RandomState>);

impl RegistryMap {
    /// Create a new empty [`RegistryMap`].
    #[must_use]
    pub fn new() -> Self { RegistryMap(IndexMap::with_hasher(RandomState::default())) }

    /// Get a reference to a [`RegistrySet`] by its [`Identifier`].
    #[inline]
    #[must_use]
    pub fn get<Q: ?Sized + Hash + Equivalent<Identifier>>(&self, id: &Q) -> Option<&RegistrySet> {
        self.0.get::<Q>(id)
    }

    /// Get a mutable reference to a [`RegistrySet`] by its [`Identifier`].
    #[inline]
    #[must_use]
    pub fn get_mut<Q: ?Sized + Hash + Equivalent<Identifier>>(
        &mut self,
        id: &Q,
    ) -> Option<&mut RegistrySet> {
        self.0.get_mut::<Q>(id)
    }

    /// Get a reference to a [`RegistrySet`] by its index.
    #[inline]
    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<(&Identifier, &RegistrySet)> {
        self.0.get_index(index)
    }

    /// Get a mutable reference to a [`RegistrySet`] by its index.
    #[inline]
    #[must_use]
    pub fn get_index_mut(&mut self, index: usize) -> Option<(&Identifier, &mut RegistrySet)> {
        self.0.get_index_mut(index)
    }

    /// Returns `true` if the map contains a [`RegistrySet`] with the given
    /// [`Identifier`].
    #[inline]
    #[must_use]
    pub fn contains<Q: ?Sized + Hash + Equivalent<Identifier>>(&self, id: &Q) -> bool {
        self.0.contains_key::<Q>(id)
    }

    /// Get the number of registries in this [`RegistryMap`].
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if the [`RegistryMap`] is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Clear all registries from this [`RegistryMap`].
    pub fn clear(&mut self) { self.0.clear(); }

    /// Get a reference to the inner [`IndexMap`] of the [`RegistryMap`].
    ///
    /// Requires calling [`RegistryMap::as_inner`] explicitly.
    #[inline]
    #[must_use]
    pub fn as_inner(map: &Self) -> &IndexMap<Identifier, RegistrySet, RandomState> { &map.0 }

    /// Get a mutable reference to the inner [`IndexMap`] of the
    /// [`RegistryMap`].
    ///
    /// Requires calling [`RegistryMap::as_inner_mut`] explicitly.
    #[inline]
    #[must_use]
    pub fn as_inner_mut(map: &mut Self) -> &mut IndexMap<Identifier, RegistrySet, RandomState> {
        &mut map.0
    }
}

// -------------------------------------------------------------------------------------------------

/// A set of [`Identifier`]s representing a single registry.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RegistrySet {
    default: Option<Identifier>,
    set: IndexSet<Identifier, RandomState>,
}

impl RegistrySet {
    /// Create a new [`RegistrySet`] from an [`IndexSet`].
    #[inline]
    #[must_use]
    pub const fn new(default: Option<Identifier>, set: IndexSet<Identifier, RandomState>) -> Self {
        RegistrySet { default, set }
    }

    /// Get a reference to a value in the set by its [`Identifier`].
    #[inline]
    #[must_use]
    pub fn get<Q: ?Sized + Hash + Equivalent<Identifier>>(&self, id: &Q) -> Option<&Identifier> {
        self.set.get::<Q>(id)
    }

    /// Get a reference to the default value of the set, if it exists.
    #[inline]
    #[must_use]
    pub fn get_default(&self) -> Option<&Identifier> { self.default.as_ref() }

    /// Get a reference to a value in the set by its [`Identifier`],
    /// or the default value if it exists.
    #[inline]
    #[must_use]
    pub fn get_or_default(&self, id: &Identifier) -> Option<&Identifier> {
        self.get(id).or(self.default.as_ref())
    }

    /// Get a reference to a value in the set by its index.
    #[inline]
    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<&Identifier> { self.set.get_index(index) }

    /// Get a reference to a value in the set by its index,
    /// or the default value if it exists.
    #[inline]
    #[must_use]
    pub fn get_index_or_default(&self, index: usize) -> Option<&Identifier> {
        self.get_index(index).or(self.default.as_ref())
    }

    /// Get the index of a value in the set by its [`Identifier`].
    #[inline]
    #[must_use]
    pub fn get_index_of<Q: ?Sized + Hash + Equivalent<Identifier>>(&self, id: &Q) -> Option<usize> {
        self.set.get_index_of::<Q>(id)
    }

    /// Get the index of a value in the set by its [`Identifier`],
    /// or the index of the default value if it exists.
    #[must_use]
    pub fn get_index_of_or_default(&self, id: &Identifier) -> Option<usize> {
        self.get_index_of(id)
            .or_else(|| self.default.as_ref().and_then(|v| self.get_index_of_or_default(v)))
    }

    /// Returns `true` if the set contains a value with the given
    /// [`Identifier`].
    #[inline]
    #[must_use]
    pub fn contains<Q: ?Sized + Hash + Equivalent<Identifier>>(&self, id: &Q) -> bool {
        self.set.contains::<Q>(id)
    }

    /// Returns `true` if the given [`Identifier`] is the default value of
    /// the set.
    ///
    /// Always returns `false` if there is no default value.
    #[inline]
    #[must_use]
    pub fn is_default<Q: ?Sized + PartialEq<Identifier>>(&self, id: &Q) -> bool {
        match &self.default {
            Some(default) => id == default,
            None => false,
        }
    }

    /// Get the number of values in this [`RegistrySet`].
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize { self.set.len() }

    /// Returns `true` if the [`RegistrySet`] is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.set.is_empty() }

    /// Clear all values from this [`RegistrySet`].
    pub fn clear(&mut self) { self.set.clear(); }

    /// Get a reference to the inner [`IndexMap`] of the [`RegistrySet`].
    ///
    /// Requires calling [`RegistrySet::as_inner`] explicitly.
    #[inline]
    #[must_use]
    pub fn as_inner(set: &Self) -> &IndexSet<Identifier, RandomState> { &set.set }

    /// Get a mutable reference to the inner [`IndexMap`] of the
    /// [`RegistrySet`].
    ///
    /// Requires calling [`RegistrySet::as_inner_mut`] explicitly.
    #[inline]
    #[must_use]
    pub fn as_inner_mut(set: &mut Self) -> &mut IndexSet<Identifier, RandomState> { &mut set.set }
}
