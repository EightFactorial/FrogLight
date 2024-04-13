use std::{marker::PhantomData, sync::Arc};

use bevy_ecs::system::Resource;
use froglight_protocol::traits::Version;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{ConvertKey, InitializeRegistry};

/// A registry that stores the default values for a specific [`Version`].
///
/// This registry is only modified by bevy [`Plugins`](bevy_app::Plugin).
#[derive(Debug, Clone, Resource)]
pub struct DefaultRegistry<V, R>
where
    V: Version,
    R: ConvertKey,
{
    storage: Arc<RwLock<Vec<R>>>,
    _version: PhantomData<V>,
}

impl<V, R> Default for DefaultRegistry<V, R>
where
    V: Version,
    R: ConvertKey + InitializeRegistry<V>,
{
    fn default() -> Self {
        Self { storage: Arc::new(RwLock::new(R::initialize())), _version: PhantomData }
    }
}

impl<V, R> DefaultRegistry<V, R>
where
    V: Version,
    R: ConvertKey,
{
    /// Creates a new [`DefaultRegistry`] with the default values.
    #[must_use]
    pub fn new() -> Self
    where
        R: InitializeRegistry<V>,
    {
        Self::default()
    }

    /// Creates a new [`SimpleRegistry`] from this [`DefaultRegistry`].
    #[must_use]
    pub fn create_simple(&self) -> SimpleRegistry<R>
    where
        R: Clone,
    {
        let storage = self.storage.read().to_vec();
        SimpleRegistry { storage: Arc::new(RwLock::new(storage)) }
    }

    /// Pushes a new value into the registry.
    pub fn push(&mut self, value: R) { self.storage.write().push(value) }

    /// Gets the value with the specified ID.
    ///
    /// This requires the value to be [`Copy`].
    #[must_use]
    pub fn get_value(&self, id: u32) -> Option<R>
    where
        R: Copy,
    {
        self.storage.read().get(id as usize).copied()
    }

    /// Gets the value with the specified ID.
    ///
    /// This requires the value to be [`Clone`].
    #[must_use]
    pub fn get_value_cloned(&self, id: u32) -> Option<R>
    where
        R: Clone,
    {
        self.storage.read().get(id as usize).cloned()
    }

    /// Gets the ID of the specified value.
    ///
    /// This requires the value to be [`PartialEq`].
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn get_id(&self, value: &R) -> Option<u32>
    where
        R: PartialEq,
    {
        self.storage.read().iter().position(|v| v == value).map(|i| i as u32)
    }

    /// Gets a [`RwLockReadGuard`] to the registry values.
    ///
    /// This is useful for bulk operations.
    ///
    /// If you need a single value, use [`DefaultRegistry::get_value`] or
    /// [`DefaultRegistry::get_value_cloned`].
    ///
    /// ---
    ///
    /// [`Note`](RwLock::read): This may cause a deadlock if the lock is not
    /// released.
    pub fn read(&self) -> RwLockReadGuard<'_, Vec<R>> { self.storage.read() }

    /// Gets a [`RwLockWriteGuard`] to the registry values.
    ///
    /// This is useful for bulk operations.
    ///
    /// If you need to push a single value, use [`DefaultRegistry::push`].
    ///
    /// ---
    ///
    /// [`Note`](RwLock::write): This may cause a deadlock if the lock is not
    /// released.
    pub fn write(&self) -> RwLockWriteGuard<'_, Vec<R>> { self.storage.write() }
}

/// A registry that stores the currently active registry values.
#[derive(Debug, Clone, Resource)]
pub struct SimpleRegistry<R>
where
    R: ConvertKey,
{
    storage: Arc<RwLock<Vec<R>>>,
}

impl<R> Default for SimpleRegistry<R>
where
    R: ConvertKey,
{
    fn default() -> Self { Self { storage: Arc::new(RwLock::new(Vec::new())) } }
}

impl<R> SimpleRegistry<R>
where
    R: ConvertKey,
{
    /// Creates a new empty [`SimpleRegistry`].
    ///
    /// This is the same as [`SimpleRegistry::default`].
    #[must_use]
    pub fn new_empty() -> Self { Self::default() }

    /// Creates a new [`SimpleRegistry`] with a [`Version`]'s default values.
    #[must_use]
    pub fn from_default<V>(default: &DefaultRegistry<V, R>) -> Self
    where
        V: Version,
        R: Clone,
    {
        default.create_simple()
    }

    /// Overwrites the registry values with the default values.
    pub fn clone_default<V>(&mut self, default: &DefaultRegistry<V, R>)
    where
        V: Version,
        R: Clone,
    {
        self.storage.write().clone_from(&default.storage.read());
    }

    /// Pushes a new value into the registry.
    ///
    /// This will assign the next available ID to the value.
    pub fn push_value(&mut self, value: R) { self.storage.write().push(value) }

    /// Gets the value with the specified ID.
    ///
    /// This requires the value to be [`Copy`].
    #[must_use]
    pub fn get_value(&self, id: u32) -> Option<R>
    where
        R: Copy,
    {
        self.storage.read().get(id as usize).copied()
    }

    /// Gets the value with the specified ID.
    ///
    /// This requires the value to be [`Clone`].
    #[must_use]
    pub fn get_value_cloned(&self, id: u32) -> Option<R>
    where
        R: Clone,
    {
        self.storage.read().get(id as usize).cloned()
    }

    /// Gets the ID of the specified value.
    ///
    /// This requires the value to be [`PartialEq`].
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn get_id(&self, value: &R) -> Option<u32>
    where
        R: PartialEq,
    {
        self.storage.read().iter().position(|v| v == value).map(|i| i as u32)
    }
}
