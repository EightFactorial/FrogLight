//! TODO

use bevy_app::{App, Plugin};
use bevy_ecs::reflect::AppTypeRegistry;
use bevy_reflect::FromType;

use crate::plugin::{
    GlobalPlugins, PluginType, ReflectInventory, plugins::player_inventory::PlayerInventoryPlugin,
};

/// A [`Plugin`] that initializes the [`GlobalPlugins`]
/// [`PluginGroup`](crate::inventory::PluginGroup).
///
/// Automatically gathers all types with [`ReflectInventory`] data
/// and initializes [`InventoryPlugins`] during startup.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) { app.register_type::<PlayerInventoryPlugin>(); }

    fn finish(&self, app: &mut App) {
        let registry = app.world().resource::<AppTypeRegistry>().read();

        #[cfg(feature = "tracing")]
        tracing::debug!(target: "froglight_inventory", "Checking the `TypeRegistry` for plugins...");

        GlobalPlugins::initialize_iter(
            registry
                .iter()
                .filter_map(|ty| ty.data::<ReflectInventory>().map(|r| (ty.type_id(), r.clone()))),
        );
    }
}

// -------------------------------------------------------------------------------------------------

impl<T: PluginType> FromType<T> for ReflectInventory {
    fn from_type() -> Self { ReflectInventory::from_plugin::<T>() }
}
