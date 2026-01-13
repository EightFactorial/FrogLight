//! TODO

use bevy_app::{App, Plugin};
use bevy_ecs::reflect::AppTypeRegistry;
use foldhash::fast::RandomState;
use indexmap::IndexMap;

use crate::inventory::{InventoryPlugins, ReflectInventory};

mod reflect;

/// A [`Plugin`] that adds ...
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) {
        let registry = app.world().resource::<AppTypeRegistry>();
        let registry = &*registry.read();

        let mut plugins = IndexMap::with_hasher(RandomState::default());
        for ty in registry.iter() {
            if let Some(reflect) = ty.data::<ReflectInventory>() {
                plugins.insert(ty.type_id(), reflect.clone());
            }
        }

        InventoryPlugins::initialize(plugins);
    }
}
