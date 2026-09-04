use alloc::{boxed::Box, vec::Vec};
use core::fmt;

#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
use froglight_common::{prelude::*, types::OnceLock};

#[cfg(feature = "bevy")]
use crate::bevy::ReflectMenuGroup;
use crate::menu::{MenuGroup, MenuGroupType};

/// A [`MenuGroup`] containing all other [`MenuGroup`]s.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash, MenuGroup))]
pub struct GlobalInventory;

// -------------------------------------------------------------------------------------------------

/// A global map of all [`MenuGroup`]s, indexed by their [`TypeId`].
static GLOBAL: OnceLock<Box<[MenuGroup]>> = OnceLock::new();

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
    pub fn try_initialize(init: Vec<MenuGroup>) -> Result<(), GlobalInventoryError> {
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
                        "Found duplicate menu identifiers {} ({:?}) and {} ({:?})",
                        a.identifier(),
                        a.type_id(),
                        b.identifier(),
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

                    return Err(GlobalInventoryError::Duplicate(*b));
                }
            }
        }

        match GLOBAL.set(init.into_boxed_slice()) {
            Ok(()) => Ok(()),
            Err(..) => Err(GlobalInventoryError::Initialized),
        }
    }
}

impl MenuGroupType for GlobalInventory {
    const IDENTIFIER: &'static Ident = Ident::new_static("froglight:global");
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalInventoryError {
    /// The [`GlobalInventory`] was already initialized.
    Initialized,
    /// A duplicate [`MenuGroup`] was found during initialization.
    Duplicate(MenuGroup),
}

impl core::error::Error for GlobalInventoryError {}
impl fmt::Display for GlobalInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Initialized => write!(f, "already initialized"),
            Self::Duplicate(group) => {
                write!(f, "duplicate menu group found: \"{}\"", group.identifier)
            }
        }
    }
}
