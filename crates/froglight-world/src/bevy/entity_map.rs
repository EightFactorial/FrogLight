use bevy_ecs::{entity::EntityHash, prelude::*};
use bevy_reflect::prelude::*;
use hashbrown::{HashMap, HashSet};

use crate::prelude::ChunkPos;

/// A map from [`ChunkPos`] to a set of [`Entity`]s and vice versa.
#[derive(Debug, Default, Clone, Reflect, Resource)]
#[reflect(Debug, Default, Resource)]
pub struct EntityPositionMap {
    entities_to_chunk: HashMap<Entity, ChunkPos, EntityHash>,
    chunk_to_entities: HashMap<ChunkPos, HashSet<Entity, EntityHash>>,
}

impl EntityPositionMap {
    /// Get the list of [`Entities`](Entity) associated with a [`ChunkPos`],
    /// if any.
    pub fn get_entities(&self, position: &ChunkPos) -> impl Iterator<Item = &Entity> {
        self.chunk_to_entities.get(position).into_iter().flatten()
    }

    /// Get the set of [`Entities`](Entity) associated with a[`ChunkPos`],
    /// if any.
    #[must_use]
    pub fn get_entity_set(&self, position: &ChunkPos) -> Option<&HashSet<Entity, EntityHash>> {
        self.chunk_to_entities.get(position)
    }

    /// Get the [`ChunkPos`] associated with an [`Entity`], if any.
    #[must_use]
    pub fn get_chunk(&self, entity: Entity) -> Option<ChunkPos> {
        self.entities_to_chunk.get(&entity).copied()
    }

    /// Insert a [`ChunkPos`]-[`Entity`] association into the map.
    ///
    /// # Panics
    /// Will panic if the chunk or entity already exists in the map.
    pub fn insert(&mut self, position: ChunkPos, entity: Entity) {
        self.entities_to_chunk.insert(entity, position);
        self.chunk_to_entities.entry(position).or_default().insert(entity);
    }

    /// Remove an [`Entity`] from the map.
    pub fn remove_entity(&mut self, position: ChunkPos, entity: Entity) {
        self.entities_to_chunk.remove(&entity);
        self.chunk_to_entities.get_mut(&position).map(|entities| entities.remove(&entity));
    }

    /// Remove a [`ChunkPos`] from the map.
    ///
    /// Returns the set of [`Entities`](Entity) associated with the
    /// [`ChunkPos`], if any.
    pub fn remove_chunk(&mut self, position: ChunkPos) -> Option<HashSet<Entity, EntityHash>> {
        self.entities_to_chunk.retain(|_, &mut pos| pos != position);
        self.chunk_to_entities.remove(&position)
    }

    /// Clear the map, removing all entries. Keeps allocated memory for reuse.
    pub fn clear(&mut self) {
        self.entities_to_chunk.clear();
        self.chunk_to_entities.clear();
    }
}
