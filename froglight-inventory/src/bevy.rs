//! TODO

use bevy_app::{App, Plugin};
use bevy_ecs::reflect::AppTypeRegistry;
use bevy_reflect::FromType;
use foldhash::fast::RandomState;
use indexmap::IndexMap;

#[cfg(feature = "froglight-entity")]
use crate::plugin::plugins::entity_equipment::EntityEquipmentPlugin;
use crate::plugin::{
    GlobalPlugins, PluginType, ReflectInventory, plugins::player_inventory::PlayerInventoryPlugin,
};

/// A [`Plugin`] that initializes the [`InventoryPlugins`] registry.
///
/// Automatically gathers all types with [`ReflectInventory`] data
/// and initializes [`InventoryPlugins`] during startup.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlayerInventoryPlugin>();
        #[cfg(feature = "froglight-entity")]
        app.register_type::<EntityEquipmentPlugin>();
    }

    fn finish(&self, app: &mut App) {
        let registry = app.world().resource::<AppTypeRegistry>();
        let registry = &*registry.read();

        let mut plugins = IndexMap::with_hasher(RandomState::default());
        for ty in registry.iter() {
            if let Some(reflect) = ty.data::<ReflectInventory>() {
                plugins.insert(ty.type_id(), reflect.clone());
            }
        }

        #[cfg(feature = "tracing")]
        tracing::debug!(target: "froglight_inventory", "Discovered {} plugins from the `TypeRegistry`", plugins.len());

        GlobalPlugins::initialize(plugins);
    }
}

// -------------------------------------------------------------------------------------------------

impl<T: PluginType> FromType<T> for ReflectInventory {
    fn from_type() -> Self { ReflectInventory::from_plugin::<T>() }
}
