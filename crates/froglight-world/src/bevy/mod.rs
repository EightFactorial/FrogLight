//! TODO

use bevy_app::{App, Plugin};
use bevy_ecs::{component::ComponentId, prelude::*, world::DeferredWorld};

mod chunk_map;
pub use chunk_map::ChunkPositionMap;

mod entity_map;
pub use entity_map::EntityPositionMap;

use crate::prelude::{Chunk, ChunkPos};

/// A [`Plugin`] that adds various world-related systems and components.
///
/// # Warning
/// This plugin does not add the [`WorldPlugin::update_entity_position_map`]
/// system to the [`App`]. You must add and schedule it yourself.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        // Register the `Chunk` and `ChunkPos` components.
        app.register_type::<Chunk>().register_type::<ChunkPos>();

        // Initialize and register the `ChunkPositionMap` resource.
        app.init_resource::<ChunkPositionMap>().register_type::<ChunkPositionMap>();
        // Initialize and register the `EntityPositionMap` resource.
        app.init_resource::<EntityPositionMap>().register_type::<EntityPositionMap>();

        // Register hooks for the `Chunk` and `ChunkPos` components.
        {
            let hooks = app.world_mut().register_component_hooks::<Chunk>();
            hooks.on_add(Self::insert_hook).on_insert(Self::insert_hook);
            hooks.on_remove(Self::remove_hook).on_replace(Self::remove_hook);

            let hooks = app.world_mut().register_component_hooks::<ChunkPos>();
            hooks.on_add(Self::insert_hook).on_insert(Self::insert_hook);
            hooks.on_remove(Self::remove_hook).on_replace(Self::remove_hook);
        }
    }
}

impl WorldPlugin {
    /// A [`System`] that updates the [`EntityPositionMap`] resource.
    #[expect(clippy::type_complexity)]
    pub fn update_entity_position_map(
        query: Query<(Entity, &ChunkPos), (Changed<ChunkPos>, Without<Chunk>)>,
        mut positions: ResMut<EntityPositionMap>,
    ) {
        for (entity, position) in &query {
            // If the entity has a previous position, remove it.
            if let Some(previous) = positions.get_chunk(entity) {
                positions.remove_entity(previous, entity);
            }

            // Insert the entity's new position.
            positions.insert(*position, entity);
        }
    }

    /// Insert a [`ChunkPos`]-[`Entity`] relationship into a map.
    fn insert_hook(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        if let Ok(entity_ref) = world.get_entity(entity) {
            match entity_ref.get_components::<(&ChunkPos, Option<&Chunk>)>() {
                // If the entity has a `Chunk` component,
                // insert it into the `ChunkPositionMap`.
                Some((&position, Some(..))) => {
                    world.resource_mut::<ChunkPositionMap>().try_insert(position, entity);
                    world.resource_mut::<EntityPositionMap>().remove_entity(position, entity);
                }
                // Otherwise, insert it into the `EntityPositionMap`.
                Some((&position, None)) => {
                    world.resource_mut::<EntityPositionMap>().insert(position, entity);
                }
                None => {}
            }
        }
    }

    /// Remove a [`ChunkPos`]-[`Entity`] relationship from a map.
    fn remove_hook(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        if let Ok(entity_ref) = world.get_entity(entity) {
            match entity_ref.get_components::<(&ChunkPos, Option<&Chunk>)>() {
                // If the entity has a `Chunk` component,
                // remove it from the `ChunkPositionMap`.
                Some((&position, Some(..))) => {
                    world.resource_mut::<ChunkPositionMap>().try_remove(position, entity);
                }
                // Otherwise, remove it from the `EntityPositionMap`.
                Some((&position, None)) => {
                    world.resource_mut::<EntityPositionMap>().remove_entity(position, entity);
                }
                None => {}
            }
        }
    }
}
