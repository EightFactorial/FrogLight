use alloc::boxed::Box;
use core::any::{Any, TypeId};

use foldhash::fast::RandomState;
use froglight_item::item::Item;
use indexmap::IndexMap;

use super::plugin::InventoryResult;
use crate::inventory::InventoryPlugins;

/// TODO
#[derive(Debug)]
pub struct Inventory {
    plugin_data: IndexMap<TypeId, Box<dyn Any>, RandomState>,
}

impl Inventory {
    /// Create a new [`Inventory`].
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Get the [`Item`] in the specified slot.
    ///
    /// Returns `None` if the slot is empty or does not exist.
    #[must_use]
    pub fn get_slot(&self, slot: usize) -> Option<Item> {
        for plugin in InventoryPlugins::get_map().values() {
            match plugin.get_slot(self, slot) {
                InventoryResult::Complete(result) => return result,
                InventoryResult::Passthrough => {}
            }
        }
        None
    }

    /// Get all [`Item`]s in the [`Inventory`].
    #[must_use]
    pub fn get_slot_all(&self) -> IndexMap<usize, Item, RandomState> {
        let mut result = IndexMap::with_hasher(RandomState::default());
        for plugin in InventoryPlugins::get_map().values() {
            match plugin.get_slot_all(self) {
                InventoryResult::Complete(part) => result.extend(part),
                InventoryResult::Passthrough => {}
            }
        }
        result.sort_unstable_keys();
        result
    }

    /// Set the [`Item`] in the specified slot.
    ///
    /// Returns `true` if the item was set successfully, `false` otherwise.
    pub fn set_slot(&mut self, item: Option<Item>, slot: usize) -> bool {
        for plugin in InventoryPlugins::get_map().values() {
            match plugin.set_slot(self, item.clone(), slot) {
                InventoryResult::Complete(()) => return true,
                InventoryResult::Passthrough => {}
            }
        }
        false
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
    #[must_use]
    pub fn set_plugin_data<T: 'static>(&mut self, data: T) -> Option<T> {
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
