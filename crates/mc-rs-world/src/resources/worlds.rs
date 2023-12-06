use bevy::{
    prelude::*,
    utils::{hashbrown::hash_map::IntoIter, HashMap},
};
use mc_rs_protocol::types::position::ChunkPos;

use crate::world::Chunk;

use super::WorldType;

#[derive(Debug, Default, Clone, PartialEq, Eq, Resource)]
pub struct Worlds(HashMap<WorldType, World>);

impl Worlds {
    /// Returns whether the [`Worlds`] contains a [`World`] of the given [`WorldType`].
    #[must_use]
    pub fn contains_world(&self, world_type: &WorldType) -> bool { self.0.contains_key(world_type) }

    /// Returns whether the [`World`] of the given [`WorldType`] contains a [`Chunk`] at the given
    /// [`ChunkPos`].
    #[must_use]
    pub fn contains_chunk(&self, world_type: &WorldType, chunk_pos: &ChunkPos) -> bool {
        self.get_world(world_type)
            .map(|world| world.contains_chunk(chunk_pos))
            .unwrap_or_default()
    }

    /// Creates an empty [`World`] of the given [`WorldType`].
    pub fn insert_world(&mut self, world_type: WorldType, world: World) {
        self.0.insert(world_type, world);
    }

    pub fn insert_chunk_entity(
        &mut self,
        world_type: WorldType,
        chunk_pos: ChunkPos,
        entity: Entity,
    ) {
        if let Some(world) = self.0.get_mut(&world_type) {
            world.insert_entity(chunk_pos, entity);
        } else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            warn!("Creating new world for chunk {chunk_pos:?}");

            let mut world = World::default();
            world.insert_entity(chunk_pos, entity);
            self.0.insert(world_type, world);
        }
    }

    /// Removes the [`World`] of the given [`WorldType`].
    pub fn remove_world(&mut self, world_type: &WorldType) { self.0.remove(world_type); }

    /// Returns the [`World`] of the given [`WorldType`], if it exists.
    #[must_use]
    pub fn get_world(&self, world_type: &WorldType) -> Option<&World> { self.0.get(world_type) }

    /// Returns the [`World`] of the given [`WorldType`], if it exists.
    #[must_use]
    pub fn get_world_mut(&mut self, world_type: &WorldType) -> Option<&mut World> {
        self.0.get_mut(world_type)
    }

    /// Gets the [`Entity`] at the given [`WorldType`] and [`ChunkPos`], if it exists.
    #[must_use]
    pub fn get_entity(&self, world_type: &WorldType, chunk_pos: &ChunkPos) -> Option<&Entity> {
        self.get_world(world_type)?.get_entity(chunk_pos)
    }

    /// Gets the [`Entity`] at the given [`WorldType`] and [`ChunkPos`], if it exists.
    #[must_use]
    pub fn get_entity_mut(
        &mut self,
        world_type: &WorldType,
        chunk_pos: &ChunkPos,
    ) -> Option<&mut Entity> {
        self.get_world_mut(world_type)?.get_entity_mut(chunk_pos)
    }

    /// Gets the [`Chunk`] at the given [`WorldType`] and [`ChunkPos`], if it exists.
    ///
    /// Requires a [`Query`] for [`Chunk`].
    #[must_use]
    pub fn get_chunk<'a>(
        &self,
        world_type: &WorldType,
        chunk_pos: &ChunkPos,
        query: &'a Query<&Chunk>,
    ) -> Option<&'a Chunk> {
        self.get_world(world_type)?.get_chunk(chunk_pos, query)
    }

    /// Gets the [`Chunk`] at the given [`WorldType`] and [`ChunkPos`], if it exists.
    ///
    /// Requires a [`Query`] for [`Ref<Chunk>`].
    #[must_use]
    pub fn get_chunk_ref<'a>(
        &self,
        world_type: &WorldType,
        chunk_pos: &ChunkPos,
        query: &'a Query<Ref<Chunk>>,
    ) -> Option<Ref<'a, Chunk>> {
        self.get_world(world_type)?.get_chunk_ref(chunk_pos, query)
    }

    /// Gets the [`Chunk`] at the given [`WorldType`] and [`ChunkPos`], if it exists.
    ///
    /// Requires a [`Query`] for [`&mut Chunk`](Chunk).
    #[must_use]
    pub fn get_chunk_mut<'a>(
        &mut self,
        world_type: &WorldType,
        chunk_pos: &ChunkPos,
        query: &'a mut Query<&mut Chunk>,
    ) -> Option<Mut<'a, Chunk>> {
        self.get_world_mut(world_type)?
            .get_chunk_mut(chunk_pos, query)
    }

    /// Iterates over all [`World`]s.
    pub fn iter(&self) -> impl Iterator<Item = (&WorldType, &World)> { self.0.iter() }

    /// Iterates over all [`World`]s.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&WorldType, &mut World)> {
        self.0.iter_mut()
    }
}

impl IntoIterator for Worlds {
    type Item = (WorldType, World);
    type IntoIter = IntoIter<WorldType, World>;
    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct World(HashMap<ChunkPos, Entity>);

impl World {
    /// Returns whether the [`World`] contains a [`Chunk`] at the given [`ChunkPos`].
    #[must_use]
    pub fn contains_chunk(&self, chunk_pos: &ChunkPos) -> bool { self.0.contains_key(chunk_pos) }

    /// Inserts the given [`Entity`] at the given [`ChunkPos`].
    pub fn insert_entity(&mut self, chunk_pos: ChunkPos, entity: Entity) {
        self.0.insert(chunk_pos, entity);
    }

    /// Removes the [`Entity`] at the given [`ChunkPos`].
    pub fn remove_entity(&mut self, chunk_pos: &ChunkPos) { self.0.remove(chunk_pos); }

    /// Gets the [`Entity`] at the given [`ChunkPos`], if it exists.
    #[must_use]
    pub fn get_entity(&self, chunk_pos: &ChunkPos) -> Option<&Entity> { self.0.get(chunk_pos) }

    /// Gets the [`Entity`] at the given [`ChunkPos`], if it exists.
    #[must_use]
    pub fn get_entity_mut(&mut self, chunk_pos: &ChunkPos) -> Option<&mut Entity> {
        self.0.get_mut(chunk_pos)
    }

    /// Gets the [`Chunk`] at the given [`ChunkPos`], if it exists.
    ///
    /// Requires a [`Query`] for [`Chunk`].
    #[must_use]
    pub fn get_chunk<'a>(
        &self,
        chunk_pos: &ChunkPos,
        query: &'a Query<&Chunk>,
    ) -> Option<&'a Chunk> {
        self.get_entity(chunk_pos)
            .and_then(|entity| query.get(*entity).ok())
    }

    /// Gets the [`Chunk`] at the given [`ChunkPos`], if it exists.
    ///
    /// Requires a [`Query`] for [`Ref<Chunk>`].
    #[must_use]
    pub fn get_chunk_ref<'a>(
        &self,
        chunk_pos: &ChunkPos,
        query: &'a Query<Ref<Chunk>>,
    ) -> Option<Ref<'a, Chunk>> {
        self.get_entity(chunk_pos)
            .and_then(|entity| query.get(*entity).ok())
    }

    /// Gets the [`Chunk`] at the given [`ChunkPos`], if it exists.
    ///
    /// Requires a [`Query`] for [`&mut Chunk`](Chunk).
    #[must_use]
    pub fn get_chunk_mut<'a>(
        &mut self,
        chunk_pos: &ChunkPos,
        query: &'a mut Query<&mut Chunk>,
    ) -> Option<Mut<'a, Chunk>> {
        self.get_entity(chunk_pos)
            .and_then(|entity| query.get_mut(*entity).ok())
    }

    /// Iterates over all [`ChunkPos`]s.
    pub fn iter(&self) -> impl Iterator<Item = (&ChunkPos, &Entity)> { self.0.iter() }

    /// Iterates over all [`ChunkPos`]s.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&ChunkPos, &mut Entity)> {
        self.0.iter_mut()
    }
}

impl IntoIterator for World {
    type Item = (ChunkPos, Entity);
    type IntoIter = IntoIter<ChunkPos, Entity>;
    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}
