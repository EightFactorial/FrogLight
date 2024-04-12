use std::{marker::PhantomData, sync::Arc};

use bevy_app::{App, PreUpdate};
use bevy_ecs::{
    schedule::IntoSystemConfigs,
    system::{Res, ResMut, Resource},
    world::{FromWorld, World},
};
use froglight_protocol::traits::Version;
use parking_lot::RwLock;

use super::{DefaultRegistry, InitializeRegistry, RegistryType, RuntimeRegistry};
use crate::RegistryPreUpdateSet;

/// A simple registry that converts between registry values and their IDs.
///
/// ## Note
/// There are two types of registries:
/// - [`DefaultRegistry`]
/// - [`RuntimeRegistry`]
///
/// Most systems should use the default [`RuntimeRegistry`] type, as it allows
/// for modification by connected servers.
///
/// If you are writing a [`Plugin`](bevy_app::Plugin) that needs to modify the
/// registry values across multiple servers, use the [`DefaultRegistry`] type.
#[derive(Debug, Clone, Resource)]
pub struct SimpleRegistry<V, T, R = RuntimeRegistry>
where
    V: Version,
    T: InitializeRegistry<V>,
    R: RegistryType,
{
    storage: Arc<RwLock<Vec<T>>>,
    _version: PhantomData<V>,
    _registry: PhantomData<R>,
}

impl<V, T, R> SimpleRegistry<V, T, R>
where
    V: Version,
    T: InitializeRegistry<V>,
    R: RegistryType,
{
    /// Gets the value at the given ID.
    ///
    /// Requires the value to implement [`Copy`].
    #[must_use]
    pub fn get_value(&self, id: u32) -> Option<T>
    where
        T: Copy,
    {
        self.storage.read().get(id as usize).copied()
    }

    /// Gets the value at the given ID.
    ///
    /// Requires the value to implement [`Clone`].
    #[must_use]
    pub fn get_value_cloned(&self, id: u32) -> Option<T>
    where
        T: Clone,
    {
        self.storage.read().get(id as usize).cloned()
    }

    /// Gets the ID of the given value.
    ///
    /// Requires the value to implement [`PartialEq`].
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn get_id(&self, value: &T) -> Option<u32>
    where
        T: PartialEq,
    {
        self.storage.read().iter().position(|v| v == value).map(|id| id as u32)
    }

    /// Inserts a value into the registry and returns its ID.
    #[allow(clippy::cast_possible_truncation)]
    pub fn insert_value(&mut self, value: T) -> u32 {
        let mut storage = self.storage.write();
        storage.push(value);
        (storage.len() - 1) as u32
    }

    /// Locks the registry for [`reading`](parking_lot::RwLock::read).
    ///
    /// ## Note
    /// This is useful for bulk operations that require reading lots of registry
    /// values.
    ///
    /// If you only need to read a single value, consider using
    /// [`get_value`](Self::get_value).
    pub fn read(&self) -> parking_lot::RwLockReadGuard<'_, Vec<T>> { self.storage.read() }

    /// Locks the registry for [`writing`](parking_lot::RwLock::write).
    ///
    /// ## Note
    /// This is useful for bulk operations that require writing lots of registry
    /// values. Be careful when using this method, as it can cause deadlocks!
    ///
    /// If you only need to write a single value, consider using
    /// [`insert_value`](Self::insert_value).
    pub fn write(&mut self) -> parking_lot::RwLockWriteGuard<'_, Vec<T>> { self.storage.write() }
}

impl<V, T> Default for SimpleRegistry<V, T, DefaultRegistry>
where
    V: Version,
    T: InitializeRegistry<V>,
{
    fn default() -> Self {
        Self {
            storage: Arc::new(RwLock::new(T::initialize())),
            _version: PhantomData,
            _registry: PhantomData,
        }
    }
}

#[allow(dead_code)]
impl<V, T> SimpleRegistry<V, T, RuntimeRegistry>
where
    V: Version,
    T: Clone + InitializeRegistry<V>,
{
    /// Creates a new [`RuntimeRegistry`] from the given [`DefaultRegistry`].
    #[must_use]
    pub fn from_default(default: &SimpleRegistry<V, T, DefaultRegistry>) -> Self {
        Self::from(default)
    }

    /// Overwrites the registry values with the default values.
    pub fn overwrite_default(&mut self, default: &SimpleRegistry<V, T, DefaultRegistry>) {
        self.storage.write().clone_from(&default.storage.read());
    }

    /// Builds the registry in the given [`App`] and registers it's
    /// [`Self::reset_registry`] [`System`](bevy_ecs::system::System).
    ///
    /// This will build both the [`DefaultRegistry`] and [`RuntimeRegistry`]
    /// types.
    pub(crate) fn build(app: &mut App)
    where
        T: 'static,
    {
        app.init_resource::<Self>()
            .add_systems(PreUpdate, Self::reset_registry.in_set(RegistryPreUpdateSet));
    }

    /// Resets the registry values to their default values.
    fn reset_registry(
        mut runtime: ResMut<Self>,
        default: Res<SimpleRegistry<V, T, DefaultRegistry>>,
    ) where
        T: 'static,
    {
        runtime.overwrite_default(&*default);
    }
}

impl<V, T> From<&SimpleRegistry<V, T, DefaultRegistry>> for SimpleRegistry<V, T, RuntimeRegistry>
where
    V: Version,
    T: Clone + InitializeRegistry<V>,
{
    fn from(default: &SimpleRegistry<V, T, DefaultRegistry>) -> Self {
        // Clone the data within, do not clone the Arc.
        let default_data = default.storage.read().clone();
        Self {
            storage: Arc::new(RwLock::new(default_data)),
            _version: PhantomData,
            _registry: PhantomData,
        }
    }
}

impl<V, T> FromWorld for SimpleRegistry<V, T, RuntimeRegistry>
where
    V: Version,
    T: 'static + Clone + InitializeRegistry<V>,
{
    fn from_world(world: &mut World) -> Self {
        SimpleRegistry::<V, T, RuntimeRegistry>::from_default(
            &*world.get_resource_or_insert_with(SimpleRegistry::<V, T, DefaultRegistry>::default),
        )
    }
}
