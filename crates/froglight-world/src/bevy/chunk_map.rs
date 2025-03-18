use bevy_app::App;
use bevy_ecs::{component::ComponentId, entity::EntityHash, prelude::*, world::DeferredWorld};
use bevy_reflect::prelude::*;
use hashbrown::HashMap;

use crate::prelude::ChunkPos;

pub(super) fn build(app: &mut App) {
    // Initialize the `ChunkMap` resource.
    app.init_resource::<ChunkMap>();

    // Register the `ChunkMap` and `ChunkPos` types.
    app.register_type::<ChunkMap>().register_type::<ChunkPos>();

    // Register component hooks for the `ChunkPos` component.
    let hooks = app.world_mut().register_component_hooks::<ChunkPos>();
    hooks.on_add(ChunkMap::insert_hook).on_insert(ChunkMap::insert_hook);
    hooks.on_remove(ChunkMap::remove_hook).on_replace(ChunkMap::remove_hook);
}

/// A map from [`ChunkPos`] to [`Entity`] and vice versa.
#[derive(Debug, Default, Clone, Reflect, Resource)]
#[reflect(Debug, Default, Resource)]
pub struct ChunkMap {
    entity_to_chunk: HashMap<Entity, ChunkPos, EntityHash>,
    chunk_to_entity: HashMap<ChunkPos, Entity>,
}

impl ChunkMap {
    /// Get the [`Entity`] associated with a [`ChunkPos`], if any.
    #[must_use]
    pub fn get_entity(&self, chunk: &ChunkPos) -> Option<Entity> {
        self.chunk_to_entity.get(chunk).copied()
    }

    /// Get the [`ChunkPos`] associated with an [`Entity`], if any.
    #[must_use]
    pub fn get_chunk(&self, entity: Entity) -> Option<ChunkPos> {
        self.entity_to_chunk.get(&entity).copied()
    }

    /// Insert a [`ChunkPos`]-[`Entity`] pair into the map.
    ///
    /// # Panics
    /// Will panic if the chunk or entity already exists in the map.
    pub fn insert(&mut self, chunk: ChunkPos, entity: Entity) {
        assert!(!self.chunk_to_entity.contains_key(&chunk), "Chunk already exists in ChunkMap!");
        assert!(!self.entity_to_chunk.contains_key(&entity), "Entity already exists in ChunkMap!");

        self.entity_to_chunk.insert(entity, chunk);
        self.chunk_to_entity.insert(chunk, entity);
    }

    /// Insert a [`ChunkPos`]-[`Entity`] pair into the map.
    ///
    /// Returns `true` if the pair was inserted,
    /// `false` if the chunk or entity already exists in the map.
    pub fn try_insert(&mut self, chunk: ChunkPos, entity: Entity) -> bool {
        if !self.chunk_to_entity.contains_key(&chunk) && !self.entity_to_chunk.contains_key(&entity)
        {
            self.entity_to_chunk.insert(entity, chunk);
            self.chunk_to_entity.insert(chunk, entity);
            true
        } else {
            false
        }
    }

    /// Remove a [`ChunkPos`]-[`Entity`] pair from the map.
    ///
    /// # Panics
    /// Will panic if the chunk and entity are not associated.
    pub fn remove(&mut self, chunk: ChunkPos, entity: Entity) {
        let removed = self.chunk_to_entity.remove(&chunk);
        removed
            .inspect(|rem| debug_assert_eq!(rem, &entity, "Entity does not match expected value!"));

        let removed = self.entity_to_chunk.remove(&entity);
        removed
            .inspect(|rem| debug_assert_eq!(rem, &chunk, "Chunk does not match expected value!"));
    }

    /// Clear the map, removing all entries. Keeps allocated memory for reuse.
    pub fn clear(&mut self) {
        self.entity_to_chunk.clear();
        self.chunk_to_entity.clear();
    }
}

impl ChunkMap {
    /// Insert a [`ChunkPos`]-[`Entity`] pair into the map.
    fn insert_hook(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        if let Some(&chunk) = world.get::<ChunkPos>(entity) {
            world.resource_mut::<Self>().try_insert(chunk, entity);
        }
    }

    /// Remove a [`ChunkPos`]-[`Entity`] pair from the map.
    fn remove_hook(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        if let Some(&chunk) = world.get::<ChunkPos>(entity) {
            world.resource_mut::<Self>().remove(chunk, entity);
        }
    }
}
