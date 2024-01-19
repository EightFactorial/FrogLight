//! A set of Worlds

use bevy_ecs::system::Resource;
use hashbrown::HashMap;

use super::{ChunkEntity, WorldChunkMap, WorldType};

/// A set of worlds.
#[derive(Debug, Clone, PartialEq, Eq, Resource)]
pub struct WorldMap(HashMap<WorldType, WorldChunkMap>);

// By default, reserve space for the 3 vanilla worlds.
impl Default for WorldMap {
    fn default() -> Self { Self(HashMap::with_capacity(3)) }
}

impl WorldMap {
    /// Create a new, empty [`WorldMap`].
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Create a new [`WorldMap`] with the specified capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self { Self(HashMap::with_capacity(capacity)) }

    /// Returns the number of elements the map can hold without reallocating.
    #[must_use]
    pub fn capacity(&self) -> usize { self.0.capacity() }

    /// Get a [`WorldChunkMap`] from the [`WorldMap`].
    #[must_use]
    pub fn get_map(&self, world: WorldType) -> Option<&WorldChunkMap> { self.0.get(&world) }

    /// Get a mutable [`WorldChunkMap`] from the [`WorldMap`].
    #[must_use]
    pub fn get_mut_map(&mut self, world: WorldType) -> Option<&mut WorldChunkMap> {
        self.0.get_mut(&world)
    }

    /// Insert a [`WorldChunkMap`] into the [`WorldMap`].
    pub fn insert_map(&mut self, world: WorldType, map: WorldChunkMap) -> Option<WorldChunkMap> {
        self.0.insert(world, map)
    }

    /// Remove a [`WorldChunkMap`] from the [`WorldMap`].
    ///
    /// If the [`WorldType`] does not exist, `None` will be returned.
    pub fn remove_map(&mut self, world: WorldType) -> Option<WorldChunkMap> {
        self.0.remove(&world)
    }

    /// Get a [`ChunkEntity`] from a [`WorldChunkMap`].
    #[must_use]
    pub fn get_chunk(&self, world: WorldType, chunk_position: u32) -> Option<&ChunkEntity> {
        self.get_map(world)?.get(chunk_position)
    }

    /// Get a mutable [`ChunkEntity`] from a [`WorldChunkMap`].
    #[must_use]
    pub fn get_mut_chunk(
        &mut self,
        world: WorldType,
        chunk_position: u32,
    ) -> Option<&mut ChunkEntity> {
        self.get_mut_map(world)?.get_mut(chunk_position)
    }

    /// Insert a [`ChunkEntity`] into a [`WorldChunkMap`].
    ///
    /// If the [`WorldType`] does not exist, it will be created.
    pub fn insert_chunk(
        &mut self,
        world: WorldType,
        chunk_position: u32,
        entity: ChunkEntity,
    ) -> Option<ChunkEntity> {
        let entry = self.0.entry(world);

        #[cfg(feature = "logging")]
        {
            if let hashbrown::hash_map::Entry::Vacant(_) = &entry {
                bevy_log::warn!(
                    "World `{}` does not exist, creating...",
                    entry.key().display_name()
                );
            }
        }

        entry.or_default().insert(chunk_position, entity)
    }

    /// Remove a [`ChunkEntity`] from a [`WorldChunkMap`].
    ///
    /// If the [`WorldType`] does not exist, `None` will be returned.
    pub fn remove_chunk(&mut self, world: WorldType, chunk_position: u32) -> Option<ChunkEntity> {
        self.get_mut_map(world)?.remove(chunk_position)
    }
}
