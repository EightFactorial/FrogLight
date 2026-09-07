use alloc::{boxed::Box, vec::Vec};
use core::fmt;

#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
use froglight_common::{prelude::*, types::OnceLock};
use froglight_item::item::Item;

#[cfg(feature = "bevy")]
use crate::bevy::ReflectMenuGroup;
use crate::{
    menu::{InventoryError, MenuGroup, MenuType},
    storage::InventoryStorage,
};

/// A [`MenuGroup`] containing all other [`MenuGroup`]s.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash, MenuGroup))]
pub struct GlobalInventory;

// -------------------------------------------------------------------------------------------------

/// A global map of all [`MenuGroup`]s, indexed by their [`TypeId`].
static GLOBAL: OnceLock<Box<[&'static MenuGroup]>> = OnceLock::new();

impl GlobalInventory {
    /// Check if the [`GlobalInventory`] has been initialized.
    #[inline]
    #[must_use]
    pub fn is_initialized() -> bool { GLOBAL.get().is_some() }

    /// Try to initialize the [`GlobalInventory`] with the given [`MenuGroup`]s.
    ///
    /// # Errors
    ///
    /// Returns the given groups if the [`GlobalInventory`] was already
    /// initialized, or if there were duplicate menu types.
    pub fn try_initialize(init: Vec<&'static MenuGroup>) -> Result<(), GlobalInventoryError> {
        // Check if it was already initialized.
        if GlobalInventory::is_initialized() {
            return Err(GlobalInventoryError::Initialized);
        }

        // Check for duplicate menus.
        for (index, a) in init.iter().enumerate() {
            for b in init.iter().skip(index + 1) {
                #[cfg(feature = "tracing")]
                if a.identifier() == b.identifier() {
                    tracing::warn!(
                        target: "froglight_inventory::global",
                        "Found duplicate menu identifiers {}: \"{:?}\" and \"{:?}\"",
                        a.identifier(),
                        a.type_id(),
                        b.type_id()
                    );
                }

                if a.type_id() == b.type_id() {
                    #[cfg(feature = "tracing")]
                    tracing::error!(
                        target: "froglight_inventory::global",
                        "Failed to initialize, found duplicate menu {} and {} (both of type {:?})",
                        a.identifier(),
                        b.identifier(),
                        b.type_id(),
                    );

                    return Err(GlobalInventoryError::Duplicate(b));
                }
            }
        }

        match GLOBAL.set(init.into_boxed_slice()) {
            Ok(()) => Ok(()),
            Err(..) => Err(GlobalInventoryError::Initialized),
        }
    }

    /// Get the global list of [`MenuGroup`]s.
    ///
    /// Returns `None` if the [`GlobalInventory`] has not been initialized.
    #[must_use]
    pub fn get_global() -> Option<&'static [&'static MenuGroup]> { GLOBAL.get().map(|s| &**s) }
}

impl MenuType for GlobalInventory {
    const IDENTIFIER: &'static Ident = Ident::new_static("froglight:global");

    fn get_slot(slot: u32, storage: &InventoryStorage) -> Result<&Item, InventoryError> {
        let global = GlobalInventory::get_global().ok_or(InventoryError::NotReady)?;

        for menu in global {
            match menu.get_slot(slot, storage) {
                Ok(item) => return Ok(item),
                Err(InventoryError::InvalidSlot) => {}
                Err(err) => return Err(err),
            }
        }

        Err(InventoryError::InvalidSlot)
    }

    fn set_slot(
        slot: u32,
        item: &Item,
        storage: &mut InventoryStorage,
    ) -> Result<Item, InventoryError> {
        let global = GlobalInventory::get_global().ok_or(InventoryError::NotReady)?;

        for menu in global {
            match menu.set_slot(slot, item, storage) {
                Ok(item) => return Ok(item),
                Err(InventoryError::InvalidSlot) => {}
                Err(err) => return Err(err),
            }
        }

        Err(InventoryError::InvalidSlot)
    }

    fn set_state(state: &Ident, storage: &mut InventoryStorage) -> Result<(), InventoryError> {
        let global = GlobalInventory::get_global().ok_or(InventoryError::NotReady)?;

        let mut result = Ok(());
        for menu in global {
            if let Err(err) = menu.set_state(state, storage) {
                result = Err(err);
            }
        }

        result
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalInventoryError {
    /// The [`GlobalInventory`] was already initialized.
    Initialized,
    /// A duplicate [`MenuGroup`] was found during initialization.
    Duplicate(&'static MenuGroup),
}

impl core::error::Error for GlobalInventoryError {}
impl fmt::Display for GlobalInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Initialized => write!(f, "already initialized"),
            Self::Duplicate(group) => {
                write!(f, "duplicate menu group found: \"{}\"", group.identifier())
            }
        }
    }
}
