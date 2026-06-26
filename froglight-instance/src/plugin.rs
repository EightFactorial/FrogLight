//! TODO

use bevy_app::{App, Plugin};
use froglight_common::prelude::{EntityId, EntityUuid};
use froglight_world::prelude::ChunkPos;

use crate::{
    instance::{
        hook::{discard_hook, insert_hook},
        reflect::ReflectSession,
    },
    prelude::SessionInstance,
};

/// A [`Plugin`] that ...
pub struct InstancePlugin;

impl Plugin for InstancePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SessionInstance>();

        app.register_type::<EntityId>().register_type_data::<EntityId, ReflectSession>();
        app.register_type::<EntityUuid>().register_type_data::<EntityUuid, ReflectSession>();
        app.register_type::<ChunkPos>().register_type_data::<ChunkPos, ReflectSession>();

        let world = app.world_mut();

        world
            .register_component_hooks::<EntityId>()
            .on_insert(insert_hook::<EntityId>)
            .on_discard(discard_hook::<EntityId>);
        world
            .register_component_hooks::<EntityUuid>()
            .on_insert(insert_hook::<EntityUuid>)
            .on_discard(discard_hook::<EntityUuid>);
        world
            .register_component_hooks::<ChunkPos>()
            .on_insert(insert_hook::<ChunkPos>)
            .on_discard(discard_hook::<ChunkPos>);
    }
}
