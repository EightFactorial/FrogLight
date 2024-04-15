use std::{hash::Hash, sync::Arc};

use bevy_ecs::system::Resource;
use froglight_protocol::traits::Version;
use hashbrown::HashMap;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::DefaultRegistry;
use crate::definitions::ConvertKey;

/// A registry that stores the currently active registry values.
///
/// Can be modified by bevy [`Plugins`](bevy_app::Plugin) and connected servers.
#[derive(Debug, Clone, Resource)]
pub struct SimpleRegistry<R>
where
    R: ConvertKey,
{
    pub(crate) id_values: Arc<RwLock<Vec<R>>>,
    pub(crate) storage: Arc<RwLock<HashMap<R, serde_json::Value>>>,
}

impl<R> Default for SimpleRegistry<R>
where
    R: ConvertKey,
{
    fn default() -> Self {
        Self {
            id_values: Arc::new(RwLock::new(Vec::new())),
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
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
    pub fn new_from_default<V>(default: &DefaultRegistry<V, R>) -> Self
    where
        V: Version,
        R: Clone,
    {
        default.create_simple()
    }

    /// Overwrites the registry values with the default values.
    pub fn overwrite_with<V>(&mut self, default: &DefaultRegistry<V, R>)
    where
        V: Version,
        R: Clone,
    {
        self.id_values.write().clone_from(&default.id_values.read());
        self.storage.write().clone_from(&default.storage.read());
    }

    /// Pushes a new value into the registry.
    ///
    /// This will assign the next available ID to the value.
    pub fn push_value(&mut self, value: R) { self.id_values.write().push(value) }

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
    /// If you need a single value, use [`SimpleRegistry::get_value`] or
    /// [`SimpleRegistry::get_value_cloned`].
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
    /// If you need to push a single value, use [`SimpleRegistry::push_value`].
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
    /// [`SimpleRegistry::write_data`] instead.
    pub fn insert_data(&mut self, value: R, data: serde_json::Value)
    where
        R: Eq + Hash,
    {
        self.storage.write().insert(value, data);
    }

    /// Gets data from the registry.
    ///
    /// If the data does not exist, this will return `None`.
    ///
    /// ---
    ///
    /// If you need to read large amounts of data, consider using
    /// [`SimpleRegistry::read_data`] instead.
    #[must_use]
    pub fn get_data(&self, value: &R) -> Option<serde_json::Value>
    where
        R: Eq + Hash,
    {
        self.storage.read().get(value).cloned()
    }

    /// Get a [`RwLockReadGuard`] to the registry data store.
    ///
    /// ---
    ///
    /// [`Note`](RwLock::read): This may cause a deadlock if the lock is not
    /// released.
    pub fn read_data(&self) -> RwLockReadGuard<'_, HashMap<R, serde_json::Value>> {
        self.storage.read()
    }

    /// Get a [`RwLockWriteGuard`] to the registry data store.
    ///
    /// ---
    ///
    /// [`Note`](RwLock::write): This may cause a deadlock if the lock is not
    /// released.
    pub fn write_data(&mut self) -> RwLockWriteGuard<'_, HashMap<R, serde_json::Value>> {
        self.storage.write()
    }
}
