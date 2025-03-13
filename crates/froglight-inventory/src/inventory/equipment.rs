#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_common::version::Version;

use super::InventorySlot;

/// A entity's equipment.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Default, PartialEq))]
#[cfg_attr(feature = "bevy", derive(Component), reflect(Component))]
pub struct EntityEquipment<V: Version> {
    head: InventorySlot<V>,
    chest: InventorySlot<V>,
    legs: InventorySlot<V>,
    feet: InventorySlot<V>,
}

impl<V: Version> Default for EntityEquipment<V> {
    fn default() -> Self { Self::new() }
}

impl<V: Version> EntityEquipment<V> {
    /// Create a new empty [`EquipmentInventory`].
    #[must_use]
    pub const fn new() -> Self {
        Self::new_from(
            InventorySlot::new(),
            InventorySlot::new(),
            InventorySlot::new(),
            InventorySlot::new(),
        )
    }

    /// Create a new [`EquipmentInventory`] from the given slots.
    #[must_use]
    pub const fn new_from(
        head: InventorySlot<V>,
        chest: InventorySlot<V>,
        legs: InventorySlot<V>,
        feet: InventorySlot<V>,
    ) -> Self {
        Self { head, chest, legs, feet }
    }

    /// Get the head slot.
    #[inline]
    #[must_use]
    pub fn head(&self) -> &InventorySlot<V> { &self.head }

    /// Get the head slot mutably.
    #[inline]
    #[must_use]
    pub fn head_mut(&mut self) -> &mut InventorySlot<V> { &mut self.head }

    /// Get the chest slot.
    #[inline]
    #[must_use]
    pub fn chest(&self) -> &InventorySlot<V> { &self.chest }

    /// Get the chest slot mutably.
    #[inline]
    #[must_use]
    pub fn chest_mut(&mut self) -> &mut InventorySlot<V> { &mut self.chest }

    /// Get the legs slot.
    #[inline]
    #[must_use]
    pub fn legs(&self) -> &InventorySlot<V> { &self.legs }

    /// Get the legs slot mutably.
    #[inline]
    #[must_use]
    pub fn legs_mut(&mut self) -> &mut InventorySlot<V> { &mut self.legs }

    /// Get the feet slot.
    #[inline]
    #[must_use]
    pub fn feet(&self) -> &InventorySlot<V> { &self.feet }

    /// Get the feet slot mutably.
    #[inline]
    #[must_use]
    pub fn feet_mut(&mut self) -> &mut InventorySlot<V> { &mut self.feet }
}
