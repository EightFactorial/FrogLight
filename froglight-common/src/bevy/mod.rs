//! TODO

use bevy_app::{App, Plugin};

mod hook;
use hook::{instance_insert_hook, instance_replace_hook};

pub mod relationship;
use relationship::EntityOfInstance;

pub mod world;
use world::WorldInstance;

use crate::entity::{EntityId, EntityUuid};

/// A [`Plugin`] that adds entity-world instance relationships.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EntityId>().register_type::<EntityUuid>();
        app.register_type::<EntityOfInstance>().register_type::<WorldInstance>();

        let world = app.world_mut();
        world
            .register_component_hooks::<EntityId>()
            .on_insert(instance_insert_hook::<EntityId>)
            .on_replace(instance_replace_hook::<EntityId>);
        world
            .register_component_hooks::<EntityUuid>()
            .on_insert(instance_insert_hook::<EntityUuid>)
            .on_replace(instance_replace_hook::<EntityUuid>);
    }
}
