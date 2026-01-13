use core::{
    any::TypeId,
    fmt::{self, Debug},
};
#[cfg(feature = "std")]
use std::sync::OnceLock;

use foldhash::fast::RandomState;
use froglight_common::prelude::Identifier;
use indexmap::IndexMap;
#[cfg(all(feature = "once_cell", not(feature = "std")))]
use once_cell::sync::OnceCell as OnceLock;

use crate::inventory::{Inventory, ReflectInventory};

static INSTANCE: OnceLock<IndexMap<TypeId, ReflectInventory, RandomState>> = OnceLock::new();

/// A global registry of inventory plugins.
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InventoryPlugins;

impl InventoryPlugins {
    /// Get access to a specific inventory plugin by its type.
    ///
    /// Returns `None` if the plugin has not been registered.
    #[must_use]
    pub fn get<T: InventoryPluginType>() -> Option<&'static ReflectInventory> {
        Self::try_get_map().and_then(|map| map.get(&TypeId::of::<T>()))
    }

    /// Get access to the global inventory plugins registry.
    ///
    /// # Panics
    ///
    /// Panics if the inventory plugins have not been initialized.
    #[must_use]
    pub fn get_map() -> &'static IndexMap<TypeId, ReflectInventory, RandomState> {
        Self::try_get_map().expect("InventoryPlugins have not been initialized!")
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
    pub fn initialize(plugins: IndexMap<TypeId, ReflectInventory, RandomState>) {
        // TODO: Sort plugins, using internal priority system?
        INSTANCE.set(plugins).expect("InventoryPlugins have already been initialized!");
    }
}

impl Debug for InventoryPlugins {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("InventoryPlugins");
        if let Some(plugins) = Self::try_get_map() {
            for (index, plugin) in plugins.values().enumerate() {
                debug.field(&alloc::string::ToString::to_string(&index), plugin.identifier());
            }
        }
        debug.finish()
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait implemented by inventory plugins.
pub trait InventoryPluginType: 'static {
    /// The identifier of this inventory plugin.
    const IDENTIFIER: Identifier<'static>;

    /// Initialize the given [`Inventory`] with this plugin's data.
    fn initialize(inventory: &mut Inventory);

    /// Handle a [`PluginEvent`].
    fn event_handle(event: PluginEvent, inventory: &mut Inventory) -> PluginResult;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PluginEvent {
    /// A request to pick an item from the given slot.
    PickItem { slot: usize },
    /// A request to place an item into the given slot.
    PlaceItem { slot: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PluginResult {
    /// Pass the event to the next plugin.
    Pass(PluginEvent),
    /// Return a response and stop processing the event.
    Complete(PluginResponse),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PluginResponse {
    /// The event was accepted.
    Accepted,
    /// The event was rejected.
    Rejected,
    /// The event could not be processed.
    ///
    /// Considered equivalent to [`PluginResponse::Rejected`].
    None,
}
