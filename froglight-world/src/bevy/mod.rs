//! TODO

use bevy_app::{App, Plugin};
use froglight_common::prelude::WorldInstance;

mod hook;
use hook::{instance_insert_hook, instance_replace_hook};

pub mod relationship;
use relationship::ChunkOfInstance;

pub mod world;
use world::WorldInstanceChunks;

use crate::prelude::ChunkPos;

/// A [`Plugin`] that adds [`ChunkPos`]-[`WorldInstance`] relationships.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ChunkOfInstance>()
            .register_type::<WorldInstanceChunks>()
            .register_type::<WorldInstance>();

        app.world_mut()
            .register_component_hooks::<ChunkPos>()
            .on_insert(instance_insert_hook)
            .on_replace(instance_replace_hook);
    }
}
