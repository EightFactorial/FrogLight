use std::ops::{Index, IndexMut};

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use froglight_common::version::Version;

use crate::slot::InventorySlot;

/// A entity's equipment.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Default, Clone, PartialEq))]
#[cfg_attr(all(feature = "bevy", feature = "reflect"), reflect(Component))]
pub struct EntityEquipment<V: Version>([InventorySlot<V>; 4]);

impl<V: Version> Default for EntityEquipment<V> {
    fn default() -> Self { Self::new() }
}

impl<V: Version> EntityEquipment<V> {
    /// Create a new empty [`EntityEquipment`].
    #[must_use]
    pub const fn new() -> Self {
        Self::new_from([
            InventorySlot::new_empty(),
            InventorySlot::new_empty(),
            InventorySlot::new_empty(),
            InventorySlot::new_empty(),
        ])
    }

    /// Create a new [`EntityEquipment`] from the given slots.
    #[must_use]
    pub const fn new_from(equipment: [InventorySlot<V>; 4]) -> Self { Self(equipment) }

    /// Get the head slot.
    #[inline]
    #[must_use]
    pub fn head(&self) -> &InventorySlot<V> { &self.0[0] }

    /// Get the head slot mutably.
    #[inline]
    #[must_use]
    pub fn head_mut(&mut self) -> &mut InventorySlot<V> { &mut self.0[0] }

    /// Get the chest slot.
    #[inline]
    #[must_use]
    pub fn chest(&self) -> &InventorySlot<V> { &self.0[1] }

    /// Get the chest slot mutably.
    #[inline]
    #[must_use]
    pub fn chest_mut(&mut self) -> &mut InventorySlot<V> { &mut self.0[1] }

    /// Get the legs slot.
    #[inline]
    #[must_use]
    pub fn legs(&self) -> &InventorySlot<V> { &self.0[2] }

    /// Get the legs slot mutably.
    #[inline]
    #[must_use]
    pub fn legs_mut(&mut self) -> &mut InventorySlot<V> { &mut self.0[2] }

    /// Get the feet slot.
    #[inline]
    #[must_use]
    pub fn feet(&self) -> &InventorySlot<V> { &self.0[3] }

    /// Get the feet slot mutably.
    #[inline]
    #[must_use]
    pub fn feet_mut(&mut self) -> &mut InventorySlot<V> { &mut self.0[3] }
}

impl<V: Version> Index<usize> for EntityEquipment<V> {
    type Output = InventorySlot<V>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            index @ 0..=3 => &self.0[index],
            _ => panic!("`EntityEquipment` index out of bounds"),
        }
    }
}
impl<V: Version> IndexMut<usize> for EntityEquipment<V> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            index @ 0..=3 => &mut self.0[index],
            _ => panic!("`EntityEquipment` index out of bounds"),
        }
    }
}
