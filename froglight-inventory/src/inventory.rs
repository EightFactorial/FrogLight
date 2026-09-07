//! TODO

use core::any::TypeId;

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
use froglight_common::prelude::*;
use froglight_item::item::Item;

use crate::{
    menu::{GlobalInventory, InventoryError, MenuGroup, MenuType},
    storage::InventoryStorage,
};

/// An inventory that can hold items.
///
/// Uses internal plugins to manage slots and menus.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Debug, Default, Clone, PartialEq, Component))]
pub struct Inventory {
    group: &'static MenuGroup,
    storage: InventoryStorage,
}

impl Default for Inventory {
    #[inline]
    fn default() -> Self { Self::new() }
}

impl Inventory {
    /// Create a new, empty [`Inventory`].
    ///
    /// Uses [`GlobalInventory`] as the default [`MenuGroupType`].
    #[inline]
    #[must_use]
    pub const fn new() -> Self { Self::new_using::<GlobalInventory>() }

    /// Create a new, empty [`Inventory`] of the given [`MenuGroupType`].
    #[inline]
    #[must_use]
    pub const fn new_using<G: MenuType>() -> Self {
        Self::new_from(G::MENU_GROUP, InventoryStorage::new())
    }

    /// Create a new, empty [`Inventory`] using the given [`MenuGroup`].
    #[inline]
    #[must_use]
    pub const fn new_from(group: &'static MenuGroup, storage: InventoryStorage) -> Self {
        Self { group, storage }
    }

    /// Get a slot in this inventory.
    ///
    /// # Errors
    ///
    /// Returns an error if the slot cannot be accessed.
    #[inline]
    pub fn get_slot(&self, slot: u32) -> Result<&Item, InventoryError> {
        self.group.get_slot(slot, &self.storage)
    }

    /// Set a slot in this inventory.
    ///
    /// # Errors
    ///
    /// Returns an error if the slot cannot be accessed.
    #[inline]
    pub fn set_slot(&mut self, slot: u32, item: &Item) -> Result<Item, InventoryError> {
        self.group.set_slot(slot, item, &mut self.storage)
    }

    /// Set the state of this inventory.
    ///
    /// # Errors
    ///
    /// Returns an error if the state cannot be set.
    #[inline]
    pub fn set_state(&mut self, state: &Ident) -> Result<(), InventoryError> {
        self.group.set_state(state, &mut self.storage)
    }

    /// Get the [`Identifier`] of this inventory's [`MenuGroup`].
    ///
    /// # Note
    ///
    /// This is only useful for debugging purposes.
    #[inline]
    #[must_use]
    pub fn group_identifier(inv: &Inventory) -> &'static Ident { inv.group.identifier() }

    /// Get the [`TypeId`] of this inventory's [`MenuGroup`].
    ///
    /// # Note
    ///
    /// This is only useful for debugging purposes.
    #[inline]
    #[must_use]
    pub fn group_type(inv: &Inventory) -> TypeId { inv.group.type_id() }
}
