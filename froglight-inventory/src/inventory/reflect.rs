use froglight_common::prelude::Identifier;

use crate::inventory::{
    Inventory, InventoryPluginType,
    plugin::{PluginEvent, PluginResult},
};

/// A collection of function pointers for interacting with an
/// [`Inventory`](crate::inventory::Inventory).
#[derive(Debug, Clone)]
pub struct ReflectInventory {
    identifier: Identifier<'static>,
    initialize: fn(&mut Inventory),
    event_handle: fn(PluginEvent, &mut Inventory) -> PluginResult,
}

impl ReflectInventory {
    /// Creates a new [`ReflectInventory`] from the given plugin type.
    #[must_use]
    pub fn from_plugin<P: InventoryPluginType>() -> Self {
        Self { identifier: P::IDENTIFIER, initialize: P::initialize, event_handle: P::event_handle }
    }

    /// Get the [`Identifier`] of this inventory plugin.
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'static> { &self.identifier }

    /// Initialize the given [`Inventory`] with this plugin's data.
    #[inline]
    pub fn initialize(&self, inventory: &mut Inventory) { (self.initialize)(inventory); }

    /// Handle a plugin event.
    #[inline]
    pub fn handle_event(&self, event: PluginEvent, inventory: &mut Inventory) -> PluginResult {
        (self.event_handle)(event, inventory)
    }
}
