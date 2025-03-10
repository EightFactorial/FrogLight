//! TODO

use std::ops::{Index, IndexMut};

use froglight_common::version::Version;

use super::InventoryQuery;
use crate::prelude::{EquipmentInventory, InventorySlot, PlayerInventory, PlayerInventoryMenu};

impl<V: Version> InventoryQuery for &'static PlayerInventory<V> {
    type InventoryId = ();
    type InventoryResult<'query> = PlayerInventoryQuery<'query, V>;
    type WorldQuery = (
        &'static PlayerInventoryMenu<V>,
        &'static EquipmentInventory<V>,
        &'static PlayerInventory<V>,
    );

    fn access(query: &mut Self::WorldQuery, (): ()) -> Self::InventoryResult<'_> {
        PlayerInventoryQuery { menu: query.0, equipment: query.1, inventory: query.2 }
    }
}

impl<V: Version> InventoryQuery for &'static PlayerInventoryMenu<V> {
    type InventoryId = ();
    type InventoryResult<'query> = PlayerInventoryQuery<'query, V>;
    type WorldQuery = (
        &'static PlayerInventoryMenu<V>,
        &'static EquipmentInventory<V>,
        &'static PlayerInventory<V>,
    );

    fn access(query: &mut Self::WorldQuery, (): ()) -> Self::InventoryResult<'_> {
        PlayerInventoryQuery { menu: query.0, equipment: query.1, inventory: query.2 }
    }
}

/// A [`Query`](bevy_ecs::system::Query) for a player's inventory.
pub struct PlayerInventoryQuery<'query, V: Version> {
    menu: &'query PlayerInventoryMenu<V>,
    equipment: &'query EquipmentInventory<V>,
    inventory: &'query PlayerInventory<V>,
}

impl<'query, V: Version> PlayerInventoryQuery<'query, V> {
    /// Access the player's inventory menu.
    #[inline]
    #[must_use]
    pub fn menu(&self) -> &'query PlayerInventoryMenu<V> { self.menu }

    /// Access the player's equipment.
    #[inline]
    #[must_use]
    pub fn equipment(&self) -> &'query EquipmentInventory<V> { self.equipment }

    /// Access the player's inventory.
    #[inline]
    #[must_use]
    pub fn inventory(&self) -> &'query PlayerInventory<V> { self.inventory }
}

impl<'query, V: Version> Index<usize> for PlayerInventoryQuery<'query, V> {
    type Output = InventorySlot<V>;

    fn index(&self, index: usize) -> &'query Self::Output {
        match index {
            // Crafting slots
            0 => self.menu().crafting_result(),
            1..=4 => &self.menu().crafting()[index - 1],
            // Equipment slots
            index @ 5..=8 => match index - 5 {
                0 => self.equipment().head(),
                1 => self.equipment().chest(),
                2 => self.equipment().legs(),
                3 => self.equipment().feet(),
                _ => unreachable!("Index cannot be greater than 3"),
            },
            // Inventory slots
            index @ 9..=35 => &self.inventory().inventory()[index - 9],
            index @ 36..=44 => &self.inventory().hotbar()[index - 36],
            // Offhand slot
            45 => self.menu().offhand(),
            oob => panic!(
                "index out of bounds: The player's inventory has 45 slots but the index is \"{oob}\""
            ),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<V: Version> InventoryQuery for &'static mut PlayerInventory<V> {
    type InventoryId = ();
    type InventoryResult<'query> = PlayerInventoryQueryMut<'query, V>;
    type WorldQuery = (
        &'static mut PlayerInventoryMenu<V>,
        &'static mut EquipmentInventory<V>,
        &'static mut PlayerInventory<V>,
    );

    fn access(query: &mut Self::WorldQuery, (): ()) -> Self::InventoryResult<'_> {
        PlayerInventoryQueryMut { menu: query.0, equipment: query.1, inventory: query.2 }
    }
}

impl<V: Version> InventoryQuery for &'static mut PlayerInventoryMenu<V> {
    type InventoryId = ();
    type InventoryResult<'query> = PlayerInventoryQueryMut<'query, V>;
    type WorldQuery = (
        &'static mut PlayerInventoryMenu<V>,
        &'static mut EquipmentInventory<V>,
        &'static mut PlayerInventory<V>,
    );

    fn access(query: &mut Self::WorldQuery, (): ()) -> Self::InventoryResult<'_> {
        PlayerInventoryQueryMut { menu: query.0, equipment: query.1, inventory: query.2 }
    }
}

/// A mutable [`Query`](bevy_ecs::system::Query) for a player's inventory.
pub struct PlayerInventoryQueryMut<'query, V: Version> {
    menu: &'query mut PlayerInventoryMenu<V>,
    equipment: &'query mut EquipmentInventory<V>,
    inventory: &'query mut PlayerInventory<V>,
}

impl<V: Version> PlayerInventoryQueryMut<'_, V> {
    /// Access the player's inventory menu.
    #[inline]
    #[must_use]
    pub fn menu(&self) -> &PlayerInventoryMenu<V> { self.menu }

    /// Access the player's inventory menu mutably.
    #[inline]
    #[must_use]
    pub fn menu_mut(&mut self) -> &mut PlayerInventoryMenu<V> { self.menu }

    /// Access the player's equipment.
    #[inline]
    #[must_use]
    pub fn equipment(&self) -> &EquipmentInventory<V> { self.equipment }

    /// Access the player's equipment mutably.
    #[inline]
    #[must_use]
    pub fn equipment_mut(&mut self) -> &mut EquipmentInventory<V> { self.equipment }

    /// Access the player's inventory.
    #[inline]
    #[must_use]
    pub fn inventory(&self) -> &PlayerInventory<V> { self.inventory }

    /// Access the player's inventory mutably.
    #[inline]
    #[must_use]
    pub fn inventory_mut(&mut self) -> &mut PlayerInventory<V> { self.inventory }
}

impl<V: Version> Index<usize> for PlayerInventoryQueryMut<'_, V> {
    type Output = InventorySlot<V>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            // Crafting slots
            0 => self.menu().crafting_result(),
            1..=4 => &self.menu().crafting()[index - 1],
            // Equipment slots
            index @ 5..=8 => match index - 5 {
                0 => self.equipment().head(),
                1 => self.equipment().chest(),
                2 => self.equipment().legs(),
                3 => self.equipment().feet(),
                _ => unreachable!("Index cannot be greater than 3"),
            },
            // Inventory slots
            index @ 9..=35 => &self.inventory().inventory()[index - 9],
            index @ 36..=44 => &self.inventory().hotbar()[index - 36],
            // Offhand slot
            45 => self.menu().offhand(),
            oob => panic!(
                "index out of bounds: The player's inventory has 45 slots but the index is \"{oob}\""
            ),
        }
    }
}
impl<V: Version> IndexMut<usize> for PlayerInventoryQueryMut<'_, V> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            // Crafting slots
            0 => self.menu_mut().crafting_result_mut(),
            1..=4 => &mut self.menu_mut().crafting_mut()[index - 1],
            // Equipment slots
            index @ 5..=8 => match index - 5 {
                0 => self.equipment_mut().head_mut(),
                1 => self.equipment_mut().chest_mut(),
                2 => self.equipment_mut().legs_mut(),
                3 => self.equipment_mut().feet_mut(),
                _ => unreachable!("Index cannot be greater than 3"),
            },
            // Inventory slots
            index @ 9..=35 => &mut self.inventory_mut().inventory_mut()[index - 9],
            index @ 36..=44 => &mut self.inventory_mut().hotbar_mut()[index - 36],
            // Offhand slot
            45 => self.menu_mut().offhand_mut(),
            oob => panic!(
                "index out of bounds: The player's inventory has 45 slots but the index is \"{oob}\""
            ),
        }
    }
}
