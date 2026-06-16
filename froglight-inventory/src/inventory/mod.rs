//! TODO

use core::any::TypeId;

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
use froglight_common::identifier::Identifier;

use crate::menu::{GlobalInventory, MenuGroup, MenuGroupType};

mod storage;
pub use storage::InventoryStorage;

/// An inventory that can hold items.
///
/// Uses internal plugins to manage slots and menus.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component, opaque))]
pub struct Inventory {
    menus: &'static MenuGroup,
    storage: InventoryStorage,
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
    pub const fn new_using<G: MenuGroupType>() -> Self {
        Self { menus: G::GROUP, storage: InventoryStorage::new() }
    }
}

impl Inventory {
    /// Get the [`Identifier`] of this inventory's [`MenuGroup`].
    ///
    /// # Note
    ///
    /// This is only useful for debugging purposes.
    #[inline]
    #[must_use]
    pub fn group_identifier(inv: &Inventory) -> &Identifier<'static> { inv.menus.identifier() }

    /// Get the [`TypeId`] of this inventory's [`MenuGroup`].
    ///
    /// # Note
    ///
    /// This is only useful for debugging purposes.
    #[inline]
    #[must_use]
    pub fn group_type(inv: &Inventory) -> TypeId { inv.menus.type_id() }
}

// -------------------------------------------------------------------------------------------------

impl Default for Inventory {
    #[inline]
    fn default() -> Self { Self::new() }
}
