#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_common::version::Version;

use crate::slot::InventorySlot;

/// A player's inventory.
///
/// Includes the player's inventory, hotbar, and the item held by the cursor.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Default, PartialEq))]
#[cfg_attr(feature = "bevy", derive(Component), reflect(Component))]
#[cfg_attr(feature = "bevy", require(PlayerInventoryMenu<V>, super::EntityEquipment<V>))]
pub struct PlayerInventory<V: Version> {
    inventory: [InventorySlot<V>; 27],
    hotbar: [InventorySlot<V>; 9],
    cursor: InventorySlot<V>,
}

impl<V: Version> Default for PlayerInventory<V> {
    fn default() -> Self { Self::new() }
}

impl<V: Version> PlayerInventory<V> {
    /// Create a new empty [`PlayerInventory`].
    #[must_use]
    pub const fn new() -> Self {
        Self::new_from(
            [
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
            ],
            [
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
            ],
            InventorySlot::new(),
        )
    }

    /// Create a new [`PlayerInventory`] from the given slots.
    #[must_use]
    pub const fn new_from(
        inventory: [InventorySlot<V>; 27],
        hotbar: [InventorySlot<V>; 9],
        cursor: InventorySlot<V>,
    ) -> Self {
        Self { inventory, hotbar, cursor }
    }

    /// Get the inventory slots.
    #[inline]
    #[must_use]
    pub fn inventory(&self) -> &[InventorySlot<V>; 27] { &self.inventory }

    /// Get the inventory slots mutably.
    #[inline]
    #[must_use]
    pub fn inventory_mut(&mut self) -> &mut [InventorySlot<V>; 27] { &mut self.inventory }

    /// Get the hotbar slots.
    #[inline]
    #[must_use]
    pub fn hotbar(&self) -> &[InventorySlot<V>; 9] { &self.hotbar }

    /// Get the hotbar slots mutably.
    #[inline]
    #[must_use]
    pub fn hotbar_mut(&mut self) -> &mut [InventorySlot<V>; 9] { &mut self.hotbar }

    /// Get the cursor slot.
    #[inline]
    #[must_use]
    pub fn cursor(&self) -> &InventorySlot<V> { &self.cursor }

    /// Get the cursor slot mutably.
    #[inline]
    #[must_use]
    pub fn cursor_mut(&mut self) -> &mut InventorySlot<V> { &mut self.cursor }
}

// -------------------------------------------------------------------------------------------------

/// A player's inventory crafting menu.
///
/// Include's the player's crafting slots and offhand slot.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect, Component))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, PartialEq, Component))]
pub struct PlayerInventoryMenu<V: Version> {
    crafting_result: InventorySlot<V>,
    crafting: [InventorySlot<V>; 4],
    offhand: InventorySlot<V>,
}

impl<V: Version> Default for PlayerInventoryMenu<V> {
    fn default() -> Self { Self::new() }
}

impl<V: Version> PlayerInventoryMenu<V> {
    /// Create a new empty [`PlayerInventoryMenu`].
    #[must_use]
    pub const fn new() -> Self {
        Self::new_from(
            [
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
                InventorySlot::new(),
            ],
            InventorySlot::new(),
            InventorySlot::new(),
        )
    }

    /// Create a new [`PlayerInventoryMenu`] from the given slots.
    #[must_use]
    pub const fn new_from(
        crafting: [InventorySlot<V>; 4],
        crafting_result: InventorySlot<V>,
        offhand: InventorySlot<V>,
    ) -> Self {
        Self { crafting_result, crafting, offhand }
    }

    /// Get the crafting slots.
    #[inline]
    #[must_use]
    pub fn crafting(&self) -> &[InventorySlot<V>; 4] { &self.crafting }

    /// Get the crafting slots mutably.
    #[inline]
    #[must_use]
    pub fn crafting_mut(&mut self) -> &mut [InventorySlot<V>; 4] { &mut self.crafting }

    /// Get the crafting result slot.
    #[inline]
    #[must_use]
    pub fn crafting_result(&self) -> &InventorySlot<V> { &self.crafting_result }

    /// Get the crafting result slot mutably.
    #[inline]
    #[must_use]
    pub fn crafting_result_mut(&mut self) -> &mut InventorySlot<V> { &mut self.crafting_result }

    /// Get the offhand slot.
    #[inline]
    #[must_use]
    pub fn offhand(&self) -> &InventorySlot<V> { &self.offhand }

    /// Get the offhand slot mutably.
    #[inline]
    #[must_use]
    pub fn offhand_mut(&mut self) -> &mut InventorySlot<V> { &mut self.offhand }
}
