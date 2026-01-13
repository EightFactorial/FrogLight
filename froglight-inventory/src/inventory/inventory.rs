use alloc::boxed::Box;
use core::any::{Any, TypeId};

use foldhash::fast::RandomState;
use indexmap::IndexMap;

use super::plugin::{PluginResponse, PluginResult};
use crate::inventory::{InventoryPlugins, plugin::PluginEvent};

/// TODO
#[derive(Debug)]
pub struct Inventory {
    plugin_data: IndexMap<TypeId, Box<dyn Any>, RandomState>,
}

impl Inventory {
    /// Create a new [`Inventory`].
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Send an event to the [`Inventory`].
    #[must_use]
    pub fn send_event(&mut self, mut event: PluginEvent) -> PluginResponse {
        for plugin in InventoryPlugins::get_map().values() {
            match plugin.handle_event(event, self) {
                PluginResult::Pass(result) => event = result,
                PluginResult::Complete(response) => return response,
            }
        }
        PluginResponse::None
    }

    /// Get a reference to plugin data of type `T` if it exists.
    #[must_use]
    pub fn plugin_data<T: 'static>(&self) -> Option<&T> {
        self.plugin_data.get(&TypeId::of::<T>()).and_then(|b| b.downcast_ref::<T>())
    }

    /// Get a mutable reference to plugin data of type `T` if it exists.
    #[must_use]
    pub fn plugin_data_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.plugin_data.get_mut(&TypeId::of::<T>()).and_then(|b| b.downcast_mut::<T>())
    }
}

// -------------------------------------------------------------------------------------------------

impl Default for Inventory {
    fn default() -> Self {
        let mut inv = Self { plugin_data: IndexMap::with_hasher(RandomState::default()) };
        InventoryPlugins::get_map().values().for_each(|p| p.initialize(&mut inv));
        inv
    }
}
