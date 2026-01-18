use core::{
    any::TypeId,
    fmt::{self, Debug},
};
#[cfg(feature = "std")]
use std::sync::OnceLock;

#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
use foldhash::fast::RandomState;
use indexmap::IndexMap;
#[cfg(all(feature = "once_cell", not(feature = "std")))]
use once_cell::sync::OnceCell as OnceLock;

use crate::{
    inventory::PluginGroup,
    plugin::{PluginType, ReflectInventory},
};

/// A global registry of inventory plugins.
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Default, Clone))]
pub struct GlobalPlugins;

static INSTANCE: OnceLock<IndexMap<TypeId, ReflectInventory, RandomState>> = OnceLock::new();

impl GlobalPlugins {
    /// Get access to a specific inventory plugin by its type.
    ///
    /// Returns `None` if the plugin has not been registered.
    #[must_use]
    pub fn get<T: PluginType>() -> Option<&'static ReflectInventory> {
        Self::try_get_map().and_then(|map| map.get(&TypeId::of::<T>()))
    }

    /// Get access to the global inventory plugins registry.
    ///
    /// # Panics
    ///
    /// Panics if the inventory plugins have not been initialized.
    #[must_use]
    pub fn get_map() -> &'static IndexMap<TypeId, ReflectInventory, RandomState> {
        Self::try_get_map().expect("GlobalPlugins have not been initialized!")
    }

    /// Try to get access to the global inventory plugins registry.
    ///
    /// Returns `None` if the inventory plugins have not been initialized.
    #[must_use]
    pub fn try_get_map() -> Option<&'static IndexMap<TypeId, ReflectInventory, RandomState>> {
        INSTANCE.get()
    }

    /// Initialize the inventory plugins registry with the given plugins.
    ///
    /// # Panics
    ///
    /// Panics if the inventory plugins have already been initialized.
    pub fn initialize_iter(plugins: impl Iterator<Item = (TypeId, ReflectInventory)>) {
        Self::initialize(plugins.collect());
    }

    /// Initialize the inventory plugins registry with the given plugins.
    ///
    /// # Panics
    ///
    /// Panics if the inventory plugins have already been initialized.
    pub fn initialize(mut plugins: IndexMap<TypeId, ReflectInventory, RandomState>) {
        plugins.sort_unstable_by_key(|_, r| r.identifier().reborrow().into_owned());

        #[cfg(feature = "tracing")]
        for plugin in plugins.values() {
            tracing::debug!(target: "froglight_inventory", "Initializing the \"{}\" plugin", plugin.identifier());
        }

        INSTANCE.set(plugins).unwrap_or_else(|input| {
            panic!("GlobalPlugins have already been initialized:\n  Current: {:?}\n->\n  Attempted: {input:?}", Self::get_map());
        });
    }
}

impl Debug for GlobalPlugins {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("GlobalPlugins");
        if let Some(plugins) = Self::try_get_map() {
            for (index, plugin) in plugins.values().enumerate() {
                debug.field(&alloc::string::ToString::to_string(&index), plugin.identifier());
            }
        }
        debug.finish()
    }
}

impl PluginGroup for GlobalPlugins {
    fn iter_plugins(&self) -> impl Iterator<Item = &ReflectInventory> { Self::get_map().values() }
}
