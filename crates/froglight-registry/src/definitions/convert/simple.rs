use std::{marker::PhantomData, sync::Arc};

use bevy_ecs::system::Resource;
use froglight_protocol::traits::Version;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::definitions::convert::{ConvertKey, InitializeIdRegistry};

/// A registry that stores the default id values for a specific [`Version`].
///
/// Can only be modified by bevy [`Plugins`](bevy_app::Plugin).
#[derive(Debug, Clone, Resource)]
pub struct DefaultIdRegistry<V, R>
where
    V: Version,
    R: ConvertKey,
{
    storage: Arc<RwLock<Vec<R>>>,
    _version: PhantomData<V>,
}

impl<V, R> Default for DefaultIdRegistry<V, R>
where
    V: Version,
    R: ConvertKey + InitializeIdRegistry<V>,
{
    fn default() -> Self {
        Self { storage: Arc::new(RwLock::new(R::initialize())), _version: PhantomData }
    }
}

impl<V, R> DefaultIdRegistry<V, R>
where
    V: Version,
    R: ConvertKey,
{
    /// Creates a new [`DefaultIdRegistry`] with the default values.
    #[must_use]
    pub fn new() -> Self
    where
        R: InitializeIdRegistry<V>,
    {
        Self::default()
    }

    /// Creates a new [`SimpleIdRegistry`] from this [`DefaultIdRegistry`].
    #[must_use]
    pub fn create_simple(&self) -> SimpleIdRegistry<R>
    where
        R: Clone,
    {
        let storage = self.storage.read().to_vec();
        SimpleIdRegistry { storage: Arc::new(RwLock::new(storage)) }
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
    /// If you need a single value, use [`DefaultIdRegistry::get_value`] or
    /// [`DefaultIdRegistry::get_value_cloned`].
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
    /// If you need to push a single value, use [`DefaultIdRegistry::push`].
    ///
    /// ---
    ///
    /// [`Note`](RwLock::write): This may cause a deadlock if the lock is not
    /// released.
    pub fn write(&self) -> RwLockWriteGuard<'_, Vec<R>> { self.storage.write() }
}

/// A registry that stores the currently active registry id values.
///
/// Can be modified by bevy [`Plugins`](bevy_app::Plugin) and connected servers.
#[derive(Debug, Clone, Resource)]
pub struct SimpleIdRegistry<R>
where
    R: ConvertKey,
{
    storage: Arc<RwLock<Vec<R>>>,
}

impl<R> Default for SimpleIdRegistry<R>
where
    R: ConvertKey,
{
    fn default() -> Self { Self { storage: Arc::new(RwLock::new(Vec::new())) } }
}

impl<R> SimpleIdRegistry<R>
where
    R: ConvertKey,
{
    /// Creates a new empty [`SimpleIdRegistry`].
    ///
    /// This is the same as [`SimpleIdRegistry::default`].
    #[must_use]
    pub fn new_empty() -> Self { Self::default() }

    /// Creates a new [`SimpleIdRegistry`] with a [`Version`]'s default values.
    #[must_use]
    pub fn new_from_default<V>(default: &DefaultIdRegistry<V, R>) -> Self
    where
        V: Version,
        R: Clone,
    {
        default.create_simple()
    }

    /// Overwrites the registry values with the default values.
    pub fn overwrite_with<V>(&mut self, default: &DefaultIdRegistry<V, R>)
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
