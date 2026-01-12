//! TODO

use bevy_app::{App, Plugin};
use froglight_common::prelude::WorldInstance;

mod hook;
use hook::{instance_add_hook, instance_remove_hook};

pub mod relationship;
use relationship::ChunkOfInstance;

pub mod world;
use world::WorldInstanceChunks;

use crate::prelude::ChunkPos;

/// A [`Plugin`] that adds chunk-world instance relationships.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ChunkOfInstance>()
            .register_type::<WorldInstanceChunks>()
            .register_type::<WorldInstance>();

        let world = app.world_mut();
        world.register_required_components::<WorldInstance, WorldInstanceChunks>();
        world
            .register_component_hooks::<ChunkPos>()
            .on_add(instance_add_hook)
            .on_remove(instance_remove_hook);
    }
}
