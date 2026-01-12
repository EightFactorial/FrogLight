//! TODO

use bevy_app::{App, Plugin};

pub mod relationship;
use relationship::{ChunkOf, EntityOf, WorldChunks, WorldEntities};

pub mod world;
use world::{WorldInstance, register_component_hooks};

/// A [`Plugin`] that adds entity-world relationship components.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ChunkOf>()
            .register_type::<EntityOf>()
            .register_type::<WorldChunks>()
            .register_type::<WorldEntities>()
            .register_type::<WorldInstance>();

        register_component_hooks(app.world_mut());
    }
}
