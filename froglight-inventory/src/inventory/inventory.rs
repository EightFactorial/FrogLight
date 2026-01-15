use alloc::boxed::Box;
use core::any::{Any, TypeId};

#[cfg(feature = "bevy")]
use bevy_ecs::component::Component;
use foldhash::fast::RandomState;
use froglight_common::prelude::Identifier;
use froglight_item::item::Item;
use indexmap::IndexMap;

use super::plugin::InventoryResult;
use crate::inventory::InventoryPlugins;

/// An inventory that can hold items.
///
/// Uses internal plugins to manage slots and menus.
#[derive(Debug)]
#[cfg_attr(feature = "bevy", derive(Component))]
pub struct Inventory {
    plugin_data: IndexMap<TypeId, Box<dyn Any + Send + Sync>, RandomState>,
}

impl Inventory {
    /// Create a new [`Inventory`].
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Get the [`Item`] in the specified slot.
    ///
    /// Returns `None` if the slot is empty or does not exist.
    #[must_use]
    pub fn get_slot(&self, mut slot: usize) -> Option<Item> {
        for plugin in InventoryPlugins::get_map().values() {
            match plugin.get_slot(self, slot) {
                InventoryResult::Passthrough(pass) => slot = pass,
                InventoryResult::Complete(result) => return result,
            }
        }
        None
    }

    /// Set the [`Item`] in the specified slot.
    ///
    /// Returns `true` if the item was set successfully, `false` otherwise.
    pub fn set_slot(&mut self, mut item: Option<Item>, mut slot: usize) -> bool {
        for plugin in InventoryPlugins::get_map().values() {
            match plugin.set_slot(self, item, slot) {
                InventoryResult::Passthrough((pass_item, pass_slot)) => {
                    item = pass_item;
                    slot = pass_slot;
                }
                InventoryResult::Complete(()) => return true,
            }
        }
        false
    }

    /// Enable a menu within the [`Inventory`].
    ///
    /// Returns `true` if the menu was enabled successfully, `false` otherwise.
    pub fn enable_menu(&mut self, mut menu: Identifier<'static>) -> bool {
        for plugin in InventoryPlugins::get_map().values() {
            match plugin.enable_menu(self, menu) {
                InventoryResult::Passthrough(pass) => menu = pass,
                InventoryResult::Complete(()) => return true,
            }
        }
        false
    }

    /// Disable a menu within the [`Inventory`].
    ///
    /// Returns `true` if the menu was disabled successfully, `false` otherwise.
    pub fn disable_menu(&mut self, mut menu: Identifier<'static>) -> bool {
        for plugin in InventoryPlugins::get_map().values() {
            match plugin.disable_menu(self, menu) {
                InventoryResult::Passthrough(pass) => menu = pass,
                InventoryResult::Complete(()) => return true,
            }
        }
        false
    }

    /// Query the status of a menu within the [`Inventory`].
    ///
    /// Returns `Some(true)` if the menu is enabled, `Some(false)` if disabled,
    /// or `None` if the menu does not exist.
    #[must_use]
    pub fn query_menu_status(&self, mut menu: Identifier<'static>) -> Option<bool> {
        for plugin in InventoryPlugins::get_map().values() {
            match plugin.query_menu_status(self, menu) {
                InventoryResult::Passthrough(pass) => menu = pass,
                InventoryResult::Complete(result) => return Some(result),
            }
        }
        None
    }

    /// Query the slots of a menu within the [`Inventory`].
    ///
    /// Returns `None` if the menu does not exist.
    #[must_use]
    pub fn query_menu_slots(
        &self,
        mut menu: Identifier<'static>,
    ) -> Option<IndexMap<usize, Item, RandomState>> {
        for plugin in InventoryPlugins::get_map().values() {
            match plugin.query_menu_slots(self, menu) {
                InventoryResult::Passthrough(pass) => menu = pass,
                InventoryResult::Complete(result) => return Some(result),
            }
        }
        None
    }

    /// Get a reference to plugin data of type `T` if it exists.
    #[must_use]
    pub fn plugin_data_ref<T: 'static>(&self) -> Option<&T> {
        self.plugin_data.get(&TypeId::of::<T>()).and_then(|b| b.downcast_ref::<T>())
    }

    /// Get a mutable reference to plugin data of type `T` if it exists.
    #[must_use]
    pub fn plugin_data_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.plugin_data.get_mut(&TypeId::of::<T>()).and_then(|b| b.downcast_mut::<T>())
    }

    /// Set plugin data of type `T`.
    ///
    /// Returns any previous data of type `T` if it existed.
    pub fn set_plugin_data<T: Send + Sync + 'static>(&mut self, data: T) -> Option<T> {
        let previous = self.plugin_data.swap_remove(&TypeId::of::<T>());
        self.plugin_data.insert(TypeId::of::<T>(), Box::new(data));
        previous.and_then(|b| b.downcast::<T>().ok()).map(|b| *b)
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
