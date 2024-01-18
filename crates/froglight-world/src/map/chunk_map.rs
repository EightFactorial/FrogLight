//! A map of [`ChunkPositions`](ChunkPos) to [`ChunkEntities`](ChunkEntity).

use hashbrown::HashMap;

use super::ChunkEntity;

/// A map of [`ChunkPositions`](ChunkPos) to [`ChunkEntities`](ChunkEntity).
///
/// Used as a part of the [`WorldMap`](crate::WorldMap) to track which
/// [`WorldType`](crate::WorldType) a [`ChunkEntity`] belongs to.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct WorldChunkMap(HashMap<u32, ChunkEntity>);

impl WorldChunkMap {
    /// Create a new, empty [`WorldChunkMap`].
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Create a new [`WorldChunkMap`] with the specified capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self { Self(HashMap::with_capacity(capacity)) }

    /// Returns the number of elements the map can hold without reallocating.
    #[must_use]
    pub fn capacity(&self) -> usize { self.0.capacity() }

    /// Create a new [`WorldChunkMap`] able to at least
    /// hold the specified radius amount of chunks.
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::WorldChunkMap;
    ///
    /// let map = WorldChunkMap::with_radius_capacity(1);
    /// assert!(map.capacity() >= 9);
    ///
    /// let map = WorldChunkMap::with_radius_capacity(2);
    /// assert!(map.capacity() >= 25);
    ///
    /// let map = WorldChunkMap::with_radius_capacity(3);
    /// assert!(map.capacity() >= 49);
    /// ```
    #[must_use]
    pub fn with_radius_capacity(radius: u32) -> Self {
        Self::with_capacity(((radius * 2) + 1).pow(2) as usize)
    }

    /// Get a [`ChunkEntity`] from the [`WorldChunkMap`].
    #[must_use]
    pub fn get(&self, chunk_position: u32) -> Option<&ChunkEntity> { self.0.get(&chunk_position) }

    /// Get a mutable [`ChunkEntity`] from the [`WorldChunkMap`].
    #[must_use]
    pub fn get_mut(&mut self, chunk_position: u32) -> Option<&mut ChunkEntity> {
        self.0.get_mut(&chunk_position)
    }

    /// Insert a [`ChunkEntity`] into the [`WorldChunkMap`].
    pub fn insert(&mut self, chunk_position: u32, entity: ChunkEntity) -> Option<ChunkEntity> {
        self.0.insert(chunk_position, entity)
    }

    /// Remove a [`ChunkEntity`] from the [`WorldChunkMap`].
    pub fn remove(&mut self, chunk_position: u32) -> Option<ChunkEntity> {
        self.0.remove(&chunk_position)
    }
}
