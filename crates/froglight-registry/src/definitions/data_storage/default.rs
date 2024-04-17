use std::{marker::PhantomData, sync::Arc};

use bevy_ecs::system::Resource;
use froglight_protocol::{common::ResourceKey, traits::Version};
use hashbrown::HashMap;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::{traits::InitializeRegistry, SimpleRegistry};
use crate::definitions::ConvertKey;

/// A registry that stores the default registry values for a specific
/// [`Version`].
///
/// Can only be modified by bevy [`Plugins`](bevy_app::Plugin).
#[derive(Debug, Clone, Resource)]
pub struct DefaultRegistry<V, R>
where
    V: Version,
    R: ConvertKey,
{
    pub(crate) id_values: Arc<RwLock<Vec<R>>>,
    pub(crate) storage: Arc<RwLock<HashMap<ResourceKey, serde_json::Value>>>,
    _version: PhantomData<V>,
}

impl<V, R> Default for DefaultRegistry<V, R>
where
    V: Version,
    R: ConvertKey + InitializeRegistry<V>,
{
    fn default() -> Self {
        Self {
            id_values: Arc::new(RwLock::new(R::initialize_ids())),
            storage: Arc::new(RwLock::new(R::initialize_storage())),
            _version: PhantomData,
        }
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
        let id_values = self.id_values.read().clone();
        let storage = self.storage.read().clone();
        SimpleRegistry {
            id_values: Arc::new(RwLock::new(id_values)),
            storage: Arc::new(RwLock::new(storage)),
        }
    }

    /// Pushes a new value into the registry.
    pub fn push(&mut self, value: R) { self.id_values.write().push(value) }

    /// Gets the value with the specified ID.
    ///
    /// This requires the value to be [`Copy`].
    #[must_use]
    pub fn get_value(&self, id: u32) -> Option<R>
    where
        R: Copy,
    {
        self.id_values.read().get(id as usize).copied()
    }

    /// Gets the value with the specified ID.
    ///
    /// This requires the value to be [`Clone`].
    #[must_use]
    pub fn get_value_cloned(&self, id: u32) -> Option<R>
    where
        R: Clone,
    {
        self.id_values.read().get(id as usize).cloned()
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
        self.id_values.read().iter().position(|v| v == value).map(|i| i as u32)
    }

    /// Gets a [`RwLockReadGuard`] to the registry id list.
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
    pub fn read_values(&self) -> RwLockReadGuard<'_, Vec<R>> { self.id_values.read() }

    /// Gets a [`RwLockWriteGuard`] to the registry id list.
    ///
    /// This is useful for bulk operations.
    ///
    /// If you need to push a single value, use [`DefaultRegistry::push`].
    ///
    /// ---
    ///
    /// [`Note`](RwLock::write): This may cause a deadlock if the lock is not
    /// released.
    pub fn write_values(&self) -> RwLockWriteGuard<'_, Vec<R>> { self.id_values.write() }

    /// Inserts data into the registry.
    ///
    /// ---
    ///
    /// If you need to insert large amounts of data, consider using
    /// [`DefaultRegistry::write_data`] instead.
    pub fn insert_data(&mut self, key: ResourceKey, data: serde_json::Value) {
        self.storage.write().insert(key, data);
    }

    /// Gets data from the registry.
    ///
    /// If the data does not exist, this will return `None`.
    ///
    /// ---
    ///
    /// If you need to read large amounts of data, consider using
    /// [`DefaultRegistry::read_data`] instead.
    #[must_use]
    pub fn get_data(&self, key: &(impl AsRef<str> + ?Sized)) -> Option<serde_json::Value> {
        self.storage.read().get(key.as_ref()).cloned()
    }

    /// Get a [`RwLockReadGuard`] to the registry data store.
    ///
    /// ---
    ///
    /// [`Note`](RwLock::read): This may cause a deadlock if the lock is not
    /// released.
    pub fn read_data(&self) -> RwLockReadGuard<'_, HashMap<ResourceKey, serde_json::Value>> {
        self.storage.read()
    }

    /// Get a [`RwLockWriteGuard`] to the registry data store.
    ///
    /// ---
    ///
    /// [`Note`](RwLock::write): This may cause a deadlock if the lock is not
    /// released.
    pub fn write_data(&mut self) -> RwLockWriteGuard<'_, HashMap<ResourceKey, serde_json::Value>> {
        self.storage.write()
    }
}
