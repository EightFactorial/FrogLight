use bevy_ecs::{entity::EntityHash, prelude::*};
use bevy_reflect::prelude::*;
use hashbrown::HashMap;

use crate::prelude::ChunkPos;

/// A map from [`ChunkPos`] to [`Entity`] and vice versa.
#[derive(Debug, Default, Clone, Reflect, Resource)]
#[reflect(Debug, Default, Resource)]
pub struct ChunkPositionMap {
    entity_to_chunk: HashMap<Entity, ChunkPos, EntityHash>,
    chunk_to_entity: HashMap<ChunkPos, Entity>,
}

impl ChunkPositionMap {
    /// Get the [`Entity`] associated with a [`ChunkPos`], if any.
    #[must_use]
    pub fn get_entity(&self, position: ChunkPos) -> Option<Entity> {
        self.chunk_to_entity.get(&position).copied()
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
    pub fn insert(&mut self, position: ChunkPos, entity: Entity) {
        assert!(self.try_insert(position, entity), "Chunk or Entity already exists!");
    }

    /// Insert a [`ChunkPos`]-[`Entity`] pair into the map.
    ///
    /// Returns `true` if the pair was inserted,
    /// `false` if the chunk or entity already exists in the map.
    pub fn try_insert(&mut self, position: ChunkPos, entity: Entity) -> bool {
        if !self.chunk_to_entity.contains_key(&position)
            && !self.entity_to_chunk.contains_key(&entity)
        {
            self.entity_to_chunk.insert(entity, position);
            self.chunk_to_entity.insert(position, entity);
            true
        } else {
            false
        }
    }

    /// Remove a [`ChunkPos`]-[`Entity`] pair from the map.
    ///
    /// # Panics
    /// Will panic if the chunk and entity are not associated.
    pub fn remove(&mut self, position: ChunkPos, entity: Entity) {
        assert!(self.try_remove(position, entity), "Chunk and Entity are not associated!");
    }

    /// Remove a [`ChunkPos`]-[`Entity`] pair from the map
    /// if the pair is associated with each other.
    ///
    /// Returns `true` if the pair was removed,
    /// `false` if the pair was not associated.
    pub fn try_remove(&mut self, position: ChunkPos, entity: Entity) -> bool {
        if self.are_associated(position, entity) {
            self.chunk_to_entity.remove(&position);
            self.entity_to_chunk.remove(&entity);
            true
        } else {
            false
        }
    }

    /// Check if a [`ChunkPos`] and [`Entity`] are associated.
    #[must_use]
    pub fn are_associated(&self, position: ChunkPos, entity: Entity) -> bool {
        self.chunk_to_entity.get(&position) == Some(&entity)
            && self.entity_to_chunk.get(&entity) == Some(&position)
    }

    /// Clear the map, removing all entries. Keeps allocated memory for reuse.
    pub fn clear(&mut self) {
        self.entity_to_chunk.clear();
        self.chunk_to_entity.clear();
    }
}
