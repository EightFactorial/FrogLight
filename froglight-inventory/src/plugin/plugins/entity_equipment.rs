//! TODO

#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
use foldhash::fast::RandomState;
use froglight_common::prelude::Identifier;
use froglight_item::prelude::Item;
use indexmap::IndexMap;

#[cfg(feature = "bevy")]
use crate::plugin::ReflectInventory;
use crate::{
    inventory::{InventoryMut, InventoryRef, InventoryResult},
    plugin::PluginType,
};

/// An [`InventoryPluginType`] that handles an entity's equipment slots.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Inventory))]
pub struct EntityEquipmentPlugin;

/// Data stored by the [`EntityEquipmentPlugin`].
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Default, Clone))]
pub struct EntityEquipmentData {
    enabled: bool,
}

impl EntityEquipmentData {
    /// Returns `true` if the inventory is enabled.
    #[inline]
    #[must_use]
    pub const fn is_enabled(&self) -> bool { self.enabled }
}

// -------------------------------------------------------------------------------------------------

impl PluginType for EntityEquipmentPlugin {
    const IDENTIFIER: Identifier<'static> =
        Identifier::new_static("froglight:inventory/entity_equipment");

    fn initialize(_: &mut InventoryMut) {}

    fn get_slot(inventory: &InventoryRef, slot: usize) -> InventoryResult<usize, Option<Item>> {
        if let Some(data) = inventory.plugin_data_ref::<EntityEquipmentData>()
            && data.is_enabled()
        {
            // If the plugin is enabled and the slot exists, return the slot
            InventoryResult::Complete(None)
        } else {
            // Otherwise, passthrough
            InventoryResult::Passthrough(slot)
        }
    }

    fn set_slot(
        inventory: &mut InventoryMut,
        item: Option<Item>,
        slot: usize,
    ) -> InventoryResult<(Option<Item>, usize), ()> {
        if let Some(plugin) = inventory.plugin_data_mut::<EntityEquipmentData>()
            && plugin.is_enabled()
        {
            // If the plugin is enabled and the slot exists, set the slot
            InventoryResult::Complete(())
        } else {
            // Otherwise, passthrough
            InventoryResult::Passthrough((item, slot))
        }
    }

    fn enable_menu(
        inventory: &mut InventoryMut,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, ()> {
        if menu == Self::IDENTIFIER {
            if let Some(plugin) = inventory.plugin_data_mut::<EntityEquipmentData>() {
                // Enable the plugin if it already exists
                plugin.enabled = true;
            } else {
                // Initialize the plugin with empty storage
                inventory.set_plugin_data(EntityEquipmentData { enabled: true });
            }
            InventoryResult::Complete(())
        } else {
            InventoryResult::Passthrough(menu)
        }
    }

    fn disable_menu(
        inventory: &mut InventoryMut,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, ()> {
        if menu == Self::IDENTIFIER
            && let Some(plugin) = inventory.plugin_data_mut::<EntityEquipmentData>()
        {
            plugin.enabled = false;
            InventoryResult::Complete(())
        } else {
            InventoryResult::Passthrough(menu)
        }
    }

    fn query_menu_status(
        inventory: &InventoryRef,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, bool> {
        if menu == Self::IDENTIFIER
            && let Some(plugin) = inventory.plugin_data_ref::<EntityEquipmentData>()
        {
            InventoryResult::Complete(plugin.enabled)
        } else {
            InventoryResult::Passthrough(menu)
        }
    }

    fn query_menu_slots(
        inventory: &InventoryRef,
        menu: Identifier<'static>,
    ) -> InventoryResult<Identifier<'static>, IndexMap<usize, Item, RandomState>> {
        if menu == Self::IDENTIFIER
            && let Some(plugin) = inventory.plugin_data_ref::<EntityEquipmentData>()
            && plugin.enabled
        {
            InventoryResult::Complete(IndexMap::default())
        } else {
            InventoryResult::Passthrough(menu)
        }
    }
}
