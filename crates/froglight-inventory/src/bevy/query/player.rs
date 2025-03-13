//! TODO

use std::ops::{Index, IndexMut};

use bevy_ecs::world::{Mut, Ref};
use froglight_common::version::Version;

use super::InventoryQuery;
use crate::prelude::{EntityEquipment, InventorySlot, PlayerInventory, PlayerInventoryMenu};

impl<V: Version> InventoryQuery for &'static PlayerInventory<V> {
    type InventoryId = ();
    type InventoryResult<'query> = PlayerInventoryQuery<'query, V>;
    type WorldQuery = (
        Ref<'static, PlayerInventoryMenu<V>>,
        Ref<'static, EntityEquipment<V>>,
        Ref<'static, PlayerInventory<V>>,
    );

    fn access(query: &mut Self::WorldQuery, (): ()) -> Self::InventoryResult<'_> {
        PlayerInventoryQuery { menu: &query.0, equipment: &query.1, inventory: &query.2 }
    }
}

impl<V: Version> InventoryQuery for &'static PlayerInventoryMenu<V> {
    type InventoryId = ();
    type InventoryResult<'query> = PlayerInventoryQuery<'query, V>;
    type WorldQuery = (
        Ref<'static, PlayerInventoryMenu<V>>,
        Ref<'static, EntityEquipment<V>>,
        Ref<'static, PlayerInventory<V>>,
    );

    fn access(query: &mut Self::WorldQuery, (): ()) -> Self::InventoryResult<'_> {
        PlayerInventoryQuery { menu: &query.0, equipment: &query.1, inventory: &query.2 }
    }
}

/// A [`Query`](bevy_ecs::system::Query) for a player's inventory.
pub struct PlayerInventoryQuery<'query, V: Version> {
    menu: &'query Ref<'static, PlayerInventoryMenu<V>>,
    equipment: &'query Ref<'static, EntityEquipment<V>>,
    inventory: &'query Ref<'static, PlayerInventory<V>>,
}

impl<'query, V: Version> PlayerInventoryQuery<'query, V> {
    /// Access the player's inventory menu.
    #[inline]
    #[must_use]
    pub fn menu(&self) -> &'query Ref<PlayerInventoryMenu<V>> { self.menu }

    /// Access the player's equipment.
    #[inline]
    #[must_use]
    pub fn equipment(&self) -> &'query Ref<EntityEquipment<V>> { self.equipment }

    /// Access the player's inventory.
    #[inline]
    #[must_use]
    pub fn inventory(&self) -> &'query Ref<PlayerInventory<V>> { self.inventory }
}

impl<V: Version> Index<usize> for PlayerInventoryQuery<'_, V> {
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

// -------------------------------------------------------------------------------------------------

impl<V: Version> InventoryQuery for &'static mut PlayerInventory<V> {
    type InventoryId = ();
    type InventoryResult<'query> = PlayerInventoryQueryMut<'query, V>;
    type WorldQuery = (
        Mut<'static, PlayerInventoryMenu<V>>,
        Mut<'static, EntityEquipment<V>>,
        Mut<'static, PlayerInventory<V>>,
    );

    fn access(query: &mut Self::WorldQuery, (): ()) -> Self::InventoryResult<'_> {
        let (menu, equipment, inventory) = query;
        PlayerInventoryQueryMut { menu, equipment, inventory }
    }
}

impl<V: Version> InventoryQuery for &'static mut PlayerInventoryMenu<V> {
    type InventoryId = ();
    type InventoryResult<'query> = PlayerInventoryQueryMut<'query, V>;
    type WorldQuery = (
        Mut<'static, PlayerInventoryMenu<V>>,
        Mut<'static, EntityEquipment<V>>,
        Mut<'static, PlayerInventory<V>>,
    );

    fn access(query: &mut Self::WorldQuery, (): ()) -> Self::InventoryResult<'_> {
        let (menu, equipment, inventory) = query;
        PlayerInventoryQueryMut { menu, equipment, inventory }
    }
}

/// A mutable [`Query`](bevy_ecs::system::Query) for a player's inventory.
///
/// # Note
/// Using any `mut` method will trigger change detection,
/// even if the value is not changed.
pub struct PlayerInventoryQueryMut<'query, V: Version> {
    menu: &'query mut Mut<'static, PlayerInventoryMenu<V>>,
    equipment: &'query mut Mut<'static, EntityEquipment<V>>,
    inventory: &'query mut Mut<'static, PlayerInventory<V>>,
}

impl<V: Version> PlayerInventoryQueryMut<'_, V> {
    /// Access the player's inventory menu.
    #[inline]
    #[must_use]
    pub fn menu(&self) -> &PlayerInventoryMenu<V> { self.menu }

    /// Access the player's inventory menu mutably.
    #[inline]
    #[must_use]
    pub fn menu_mut(&mut self) -> &mut Mut<'static, PlayerInventoryMenu<V>> { self.menu }

    /// Access the player's equipment.
    #[inline]
    #[must_use]
    pub fn equipment(&self) -> &EntityEquipment<V> { self.equipment }

    /// Access the player's equipment mutably.
    #[inline]
    #[must_use]
    pub fn equipment_mut(&mut self) -> &mut Mut<'static, EntityEquipment<V>> { self.equipment }

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
