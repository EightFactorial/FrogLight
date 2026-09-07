//! TODO
#![allow(unpredictable_function_pointer_comparisons, reason = "Ignored")]

use core::{any::TypeId, fmt};

#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
use froglight_common::prelude::*;

mod global;
use froglight_item::item::Item;
pub use global::GlobalInventory;

use crate::storage::InventoryStorage;

/// Generic [`MenuGroupType`] data.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, opaque))]
pub struct MenuGroup {
    identifier: &'static Ident,
    type_id: TypeId,

    get_slot_fn: fn(u32, &InventoryStorage) -> Result<&Item, InventoryError>,
    set_slot_fn: fn(u32, &Item, &mut InventoryStorage) -> Result<Item, InventoryError>,
    set_state_fn: fn(&Ident, &mut InventoryStorage) -> Result<(), InventoryError>,
}

impl MenuGroup {
    /// Create a new [`MenuGroup`] of the given type.
    #[inline]
    #[must_use]
    pub const fn new<G: MenuType + ?Sized>() -> Self {
        Self {
            identifier: G::IDENTIFIER,
            type_id: TypeId::of::<G>(),
            get_slot_fn: G::get_slot,
            set_slot_fn: G::set_slot,
            set_state_fn: G::set_state,
        }
    }

    /// Get the [`Identifier`] of this group.
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &'static Ident { self.identifier }

    /// Get the [`TypeId`] of this group.
    #[inline]
    #[must_use]
    pub const fn type_id(&self) -> TypeId { self.type_id }

    /// Get the requested slot from the inventory.
    ///
    /// # Errors
    ///
    /// Returns an error if the slot cannot be accessed.
    #[inline]
    pub fn get_slot<'a>(
        &self,
        slot: u32,
        storage: &'a InventoryStorage,
    ) -> Result<&'a Item, InventoryError> {
        (self.get_slot_fn)(slot, storage)
    }

    /// Set the requested slot in the inventory.
    ///
    /// # Errors
    ///
    /// Returns an error if the slot cannot be accessed.
    #[inline]
    pub fn set_slot(
        &self,
        slot: u32,
        item: &Item,
        storage: &mut InventoryStorage,
    ) -> Result<Item, InventoryError> {
        (self.set_slot_fn)(slot, item, storage)
    }

    /// Set the state of the inventory.
    ///
    /// # Errors
    ///
    /// Returns an error if the state cannot be set.
    #[inline]
    pub fn set_state(
        &self,
        state: &Ident,
        storage: &mut InventoryStorage,
    ) -> Result<(), InventoryError> {
        (self.set_state_fn)(state, storage)
    }
}

impl fmt::Debug for MenuGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MenuGroup").field(&self.identifier).finish_non_exhaustive()
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait for inventory menus.
pub trait MenuType: 'static {
    /// The menu's unique identifier.
    const IDENTIFIER: &'static Ident;
    /// The menu's [`MenuGroup`] instance.
    const MENU_GROUP: &'static MenuGroup = &const { MenuGroup::new::<Self>() };

    /// Get the requested slot from the inventory.
    ///
    /// # Errors
    ///
    /// Returns an error if the slot cannot be interacted with.
    fn get_slot(slot: u32, storage: &InventoryStorage) -> Result<&Item, InventoryError>;

    /// Set the requested slot in the inventory, returning the previous item in
    /// that slot.
    ///
    /// # Errors
    ///
    /// Returns an error if the slot cannot be interacted with.
    fn set_slot(
        slot: u32,
        item: &Item,
        storage: &mut InventoryStorage,
    ) -> Result<Item, InventoryError>;

    /// Set the state of the inventory.
    ///
    /// # Errors
    ///
    /// Returns an error if the state cannot be set.
    fn set_state(state: &Ident, storage: &mut InventoryStorage) -> Result<(), InventoryError>;
}

/// An error that can occur when interacting with an inventory.
pub enum InventoryError {
    /// The requested slot does not exist.
    InvalidSlot,
    /// The inventory is not ready to be used yet.
    NotReady,
}
