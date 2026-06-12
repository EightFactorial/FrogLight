use alloc::vec::Vec;
use core::ops::Deref;
#[cfg(feature = "std")]
use std::sync::OnceLock;

#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
use froglight_common::prelude::Identifier;
#[cfg(not(feature = "std"))]
use once_cell::sync::OnceCell as OnceLock;

#[cfg(feature = "bevy")]
use crate::bevy::ReflectMenuGroup;
use crate::menu::{MenuGroup, MenuGroupType};

/// A [`MenuGroup`] containing all other [`MenuGroup`]s.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash, MenuGroup))]
pub struct GlobalInventory;

impl Deref for GlobalInventory {
    type Target = MenuGroup;

    fn deref(&self) -> &Self::Target { Self::GROUP }
}

// -------------------------------------------------------------------------------------------------

/// A global map of all [`MenuGroup`]s, indexed by their [`TypeId`].
static GLOBAL: OnceLock<Vec<MenuGroup>> = OnceLock::new();

impl GlobalInventory {
    /// Check if the [`GlobalInventory`] has been initialized.
    #[inline]
    #[must_use]
    pub fn initialized() -> bool { GLOBAL.get().is_some() }

    /// Try to initialize the [`GlobalInventory`] with the given [`MenuGroup`]s.
    ///
    /// # Errors
    ///
    /// Returns the given groups if the [`GlobalInventory`] was already
    /// initialized, or if there were duplicate menu types.
    pub fn try_initialize(init: Vec<MenuGroup>) -> Result<(), Vec<MenuGroup>> {
        // Check if it was already initialized.
        if GlobalInventory::initialized() {
            return Err(init);
        }

        // Check for duplicate menus.
        if init.len() > 2 {
            for (index, a) in init.iter().enumerate() {
                for b in init.iter().skip(index + 1) {
                    if a.type_id() == b.type_id() {
                        #[cfg(feature = "tracing")]
                        tracing::error!(
                            "Failed to initialize the `GlobalInventory`, found duplicate entries {:?} ({}) and {:?} ({})",
                            a.identifier().as_str(),
                            a.type_id(),
                            b.identifier().as_str(),
                            b.type_id()
                        );
                        return Err(init);
                    }
                }
            }
        }

        GLOBAL.set(init)
    }
}

impl MenuGroupType for GlobalInventory {
    const GROUP: &'static MenuGroup = const {
        // Create a static to guarantee all `GROUP` refs use the same instance.
        static INNER: MenuGroup = MenuGroup::new::<GlobalInventory>();
        &INNER
    };
    const IDENTIFIER: Identifier<'static> = Identifier::new_static("froglight:global");
}
