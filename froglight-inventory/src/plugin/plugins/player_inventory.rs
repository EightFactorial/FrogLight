//! TODO

use alloc::{vec, vec::Vec};

#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
use foldhash::fast::RandomState;
use froglight_common::prelude::Identifier;
use froglight_item::prelude::Item;
use indexmap::IndexMap;

#[cfg(feature = "bevy")]
use crate::plugin::ReflectInventory;
use crate::{
    inventory::{Inventory, InventoryResult},
    plugin::PluginType,
};

/// An [`InventoryPluginType`] that handles the player's inventory.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Inventory))]
pub struct PlayerInventoryPlugin;

/// Data stored by the [`PlayerInventoryPlugin`].
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Default, Clone))]
pub struct PlayerInventoryData {
    enabled: bool,
    #[cfg_attr(feature = "bevy", reflect(ignore))]
    storage: Vec<Option<Item>>,
}

impl PlayerInventoryData {
    const STORAGE_MAX: usize = Self::STORAGE_MIN + Self::STORAGE_SIZE;
    const STORAGE_MIN: usize = 9;
    /// Width x Height + Offhand
    const STORAGE_SIZE: usize = (4 * 9) + 1;

    /// Returns `true` if the inventory is enabled.
    #[inline]
    #[must_use]
    pub const fn is_enabled(&self) -> bool { self.enabled }

    /// Returns a reference to the raw storage vector.
    #[inline]
    #[must_use]
    pub const fn raw_storage(&self) -> &Vec<Option<Item>> { &self.storage }

    /// Returns a reference to the item in the given slot, if it exists.
    #[must_use]
    pub fn get_slot_index(&self, slot: usize) -> Option<&Option<Item>> {
        self.storage.get(Self::slot_shift(slot)?)
    }

    /// Returns a mutable reference to the item in the given slot, if it exists.
    #[must_use]
    pub fn get_slot_index_mut(&mut self, slot: usize) -> Option<&mut Option<Item>> {
        self.storage.get_mut(Self::slot_shift(slot)?)
    }

    /// Get the player's inventory slots (0-26).
    ///
    /// Returns an empty slice if no slots exist.
    #[must_use]
    pub fn get_inventory(&self) -> &[Option<Item>] { self.storage.get(0..27).unwrap_or(&[]) }

    /// Get the player's hotbar slots (27-35).
    ///
    /// Returns an empty slice if no slots exist.
    #[must_use]
    pub fn get_hotbar(&self) -> &[Option<Item>] { self.storage.get(27..36).unwrap_or(&[]) }

    /// Get the player's offhand slot (36).
    ///
    /// Returns `None` if the slot does not exist.
    #[must_use]
    pub fn get_offhand(&self) -> &Option<Item> { self.storage.get(36).unwrap_or(&None) }

    /// Shifts the given external slot to an internal index.
    ///
    /// For example: Slot 9 becomes index 0, slot 10 becomes index 1, etc.
    #[must_use]
    pub const fn slot_shift(slot: usize) -> Option<usize> {
        match slot {
            Self::STORAGE_MIN..=Self::STORAGE_MAX => Some(slot - Self::STORAGE_MIN),
            _ => None,
        }
    }

    /// Shifts the given internal index to an external slot.
    ///
    /// For example: Index 0 becomes slot 9, index 1 becomes slot 10, etc.
    #[must_use]
    pub const fn slot_unshift(index: usize) -> Option<usize> {
        match index {
            0..=Self::STORAGE_SIZE => Some(index + Self::STORAGE_MIN),
            _ => None,
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl PluginType for PlayerInventoryPlugin {
    const IDENTIFIER: Identifier<'static> =
        Identifier::new_static("froglight:inventory/player_inventory");

    fn initialize(_: &mut Inventory) {}

    fn get_slot(inventory: &Inventory, slot: usize) -> InventoryResult<usize, Option<Item>> {
        if let Some(data) = inventory.plugin_data_ref::<PlayerInventoryData>()
            && data.is_enabled()
            && let Some(slot) = data.get_slot_index(slot)
        {
            // If the plugin is enabled and the slot exists, return the slot
            InventoryResult::Complete(slot.clone())
        } else {
            // Otherwise, passthrough
            InventoryResult::Passthrough(slot)
        }
    }

    fn set_slot(
        inventory: &mut Inventory,
        item: Option<Item>,
        slot: usize,
    ) -> InventoryResult<(Option<Item>, usize), ()> {
        if let Some(plugin) = inventory.plugin_data_mut::<PlayerInventoryData>()
            && plugin.is_enabled()
            && let Some(slot) = plugin.get_slot_index_mut(slot)
        {
            // If the plugin is enabled and the slot exists, set the slot
            *slot = item;
            InventoryResult::Complete(())
        } else {
            // Otherwise, passthrough
            InventoryResult::Passthrough((item, slot))
        }
    }

    fn enable_menu(
        inventory: &mut Inventory,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, ()> {
        if menu == Self::IDENTIFIER {
            if let Some(plugin) = inventory.plugin_data_mut::<PlayerInventoryData>() {
                // Enable the plugin if it already exists
                plugin.enabled = true;
            } else {
                // Initialize the plugin with empty storage
                inventory.set_plugin_data(PlayerInventoryData {
                    enabled: true,
                    storage: vec![None; 30],
                });
            }
            InventoryResult::Complete(())
        } else {
            InventoryResult::Passthrough(menu)
        }
    }

    fn disable_menu(
        inventory: &mut Inventory,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, ()> {
        if menu == Self::IDENTIFIER
            && let Some(plugin) = inventory.plugin_data_mut::<PlayerInventoryData>()
        {
            plugin.enabled = false;
            InventoryResult::Complete(())
        } else {
            InventoryResult::Passthrough(menu)
        }
    }

    fn query_menu_status(
        inventory: &Inventory,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, bool> {
        if menu == Self::IDENTIFIER
            && let Some(plugin) = inventory.plugin_data_ref::<PlayerInventoryData>()
        {
            InventoryResult::Complete(plugin.enabled)
        } else {
            InventoryResult::Passthrough(menu)
        }
    }

    fn query_menu_slots(
        inventory: &Inventory,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, IndexMap<usize, Item, RandomState>> {
        if menu == Self::IDENTIFIER
            && let Some(plugin) = inventory.plugin_data_ref::<PlayerInventoryData>()
            && plugin.enabled
        {
            let mut slots =
                IndexMap::with_capacity_and_hasher(plugin.storage.len(), RandomState::default());
            for (index, item_option) in plugin.storage.iter().enumerate() {
                if let Some(index) = PlayerInventoryData::slot_unshift(index)
                    && let Some(item) = item_option
                {
                    slots.insert(index, item.clone());
                }
            }
            InventoryResult::Complete(slots)
        } else {
            InventoryResult::Passthrough(menu)
        }
    }
}
