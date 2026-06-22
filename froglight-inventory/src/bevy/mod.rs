//! TODO

use alloc::vec::Vec;
use core::any::TypeId;

use bevy_app::{App, Plugin};
use bevy_ecs::reflect::AppTypeRegistry;

mod reflect;
pub use reflect::ReflectMenuGroup;

use crate::{menu::GlobalInventory, prelude::Inventory};

/// A [`Plugin`] that ...
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Inventory>();
        app.register_type::<GlobalInventory>();
    }

    fn finish(&self, app: &mut App) {
        if GlobalInventory::initialized() {
            #[cfg(feature = "tracing")]
            tracing::debug!(target: "froglight_inventory", "The `GlobalInventory` is already initialized, skipping `TypeRegistry`!");
            return;
        }

        #[cfg(feature = "tracing")]
        tracing::debug!(target: "froglight_inventory", "Checking the `TypeRegistry` for inventory menus...");

        // Collect all `MenuGroup`s from the `TypeRegistry`,
        // excluding the `GlobalInventory` itself.
        let mut init = Vec::new();
        for registration in app.world().resource::<AppTypeRegistry>().read().iter() {
            if let Some(group) = registration.data::<ReflectMenuGroup>()
                && group.type_id() != TypeId::of::<GlobalInventory>()
            {
                #[cfg(feature = "tracing")]
                tracing::debug!(target: "froglight_inventory", "Found inventory menu {:?}", group.identifier().as_str());
                init.push(group.as_inner().clone());
            }
        }

        // Initialize the `GlobalInventory` with the collected groups.
        if GlobalInventory::try_initialize(init).is_ok() {
            #[cfg(feature = "tracing")]
            tracing::debug!(target: "froglight_inventory", "Initialized the `GlobalInventory`");
        } else {
            #[cfg(feature = "tracing")]
            tracing::error!(target: "froglight_inventory", "Failed to initialize the `GlobalInventory`, were there duplicate menus?");

            // Exit the app if initialization fails in debug mode.
            #[cfg(debug_assertions)]
            {
                #[cfg(feature = "tracing")]
                tracing::warn!(target: "froglight_inventory", "Triggering debug app exit...");
                app.world_mut().write_message(bevy_app::AppExit::error());
            }
        }
    }
}
