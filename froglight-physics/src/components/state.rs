//! TODO

use core::any::TypeId;

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_block::{
    block::{BlockShape, GlobalId},
    storage::BlockStorage,
};
use froglight_world::prelude::*;
use glam::IVec3;
#[cfg(feature = "std")]
use quick_cache::unsync::Cache;

#[allow(unused_imports, reason = "May be used depending on features")]
use crate::prelude::*;

/// The current state of the physics simulation for an entity.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", require(Transform, PreviousTransform))]
#[cfg_attr(feature = "bevy", require(Velocity, Acceleration, OnGround))]
#[cfg_attr(feature = "bevy", reflect(opaque, Debug, Default, Clone, Component))]
pub struct PhysicsState {
    #[cfg(feature = "std")]
    lookup_cache: Cache<(GlobalId, TypeId), &'static BlockShape<'static>>,
    world_cache: (BlockPos, [&'static BlockShape<'static>; Self::CACHE_SIZE]),
}

impl Default for PhysicsState {
    #[inline]
    fn default() -> Self {
        Self {
            #[cfg(feature = "std")]
            lookup_cache: Cache::new(Self::LOOKUP_SIZE),
            world_cache: (BlockPos::ZERO, [&BlockShape::None; _]),
        }
    }
}

impl PhysicsState {
    const CACHE_SIDE_LENGTH: usize = 5;
    /// The world cache size for block position -> block shape.
    ///
    /// At:
    ///   - 3: 3x3x3 = 27 blocks
    ///   - 5: 5x5x5 = 125 blocks
    const CACHE_SIZE: usize =
        Self::CACHE_SIDE_LENGTH * Self::CACHE_SIDE_LENGTH * Self::CACHE_SIDE_LENGTH;
    /// The lookup cache size for block id -> block shape.
    ///
    /// At:
    ///   - 3: 2x2x2 = 8 blocks
    ///   - 5: 4x4x4 = 64 blocks
    #[cfg(feature = "std")]
    const LOOKUP_SIZE: usize = (Self::CACHE_SIDE_LENGTH - 1)
        * (Self::CACHE_SIDE_LENGTH - 1)
        * (Self::CACHE_SIDE_LENGTH - 1);

    /// Create a new [`PhysicsState`].
    #[inline]
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Get the cached world shape around the given block position.
    ///
    /// If `refresh` is `true`, the cache will always be updated.
    ///
    /// # Note
    ///
    /// This should only be called using current entity's position.
    pub fn world_shape<'a>(
        &mut self,
        position: BlockPos,
        chunks: impl Fn(ChunkPos) -> (&'a NaiveChunk, &'a BlockStorage, TypeId),
        mut refresh: bool,
    ) -> &[&'static BlockShape<'static>; Self::CACHE_SIZE] {
        // If the position is different, update the cache.
        refresh |= position != self.world_cache.0;

        if refresh {
            #[expect(clippy::cast_possible_truncation, reason = "Small Constant")]
            #[expect(clippy::cast_possible_wrap, reason = "Small Constant")]
            const LENGTH: i32 = PhysicsState::CACHE_SIDE_LENGTH as i32;

            // TODO: Optimize by shifting the cache and only updating the new blocks.

            let mut chunk_pos = position.into_chunk_pos();
            let (mut chunk, mut storage, mut ty) = (chunks)(chunk_pos);

            for y in -LENGTH..LENGTH {
                for z in -LENGTH..LENGTH {
                    for x in -LENGTH..LENGTH {
                        // Get the world position of the block.
                        let block_pos = BlockPos::new_xyz(x, y, z) + position;

                        // Update `chunk`, `storage`, and `ty` if the block is in a different chunk.
                        let new_chunk_pos = block_pos.into_chunk_pos();
                        if new_chunk_pos != chunk_pos {
                            chunk_pos = new_chunk_pos;
                            (chunk, storage, ty) = (chunks)(new_chunk_pos);
                        }

                        // Update the cache.
                        let block_id = chunk.get_raw_block(block_pos).unwrap_or(0);
                        self.world_cache.1[Self::cache_index(x, y, z)] =
                            self.block_shape(GlobalId::new(block_id), storage, ty);
                    }
                }
            }
        }

        &self.world_cache.1
    }

    /// Get the index in the world cache for the given [`BlockPos`].
    ///
    /// Returns `None` if the position is outside the cache bounds.
    #[must_use]
    #[expect(clippy::cast_possible_truncation, reason = "Small Constant")]
    #[expect(clippy::cast_possible_wrap, reason = "Small Constant")]
    pub fn index_for(root: BlockPos, pos: BlockPos) -> Option<usize> {
        let offset = (pos - root).as_ivec3();
        if offset.cmpge(IVec3::splat(Self::CACHE_SIDE_LENGTH as i32)).any()
            || offset.cmple(IVec3::splat(-(Self::CACHE_SIDE_LENGTH as i32))).any()
        {
            None
        } else {
            Some(Self::cache_index(offset.x, offset.y, offset.z))
        }
    }

    /// Get the [`BlockShape`] of the given [`Block`].
    ///
    /// Uses an internal [`Cache`] to avoid redundant calls to
    /// [`Block::shape`].
    #[inline]
    #[cfg(feature = "std")]
    fn block_shape(
        &mut self,
        block_id: GlobalId,
        storage: &BlockStorage,
        storage_ver: TypeId,
    ) -> &'static BlockShape<'static> {
        self.lookup_cache
            .get_or_insert_with(&(block_id, storage_ver), || {
                use core::convert::Infallible;

                if let Some(block) = storage.get_block(block_id) {
                    Result::<_, Infallible>::Ok(block.shape())
                } else {
                    let air = storage.get_block(GlobalId::new(0)).unwrap();
                    Result::<_, Infallible>::Ok(air.shape())
                }
            })
            .unwrap()
            .unwrap()
    }

    /// Get the [`BlockShape`] of the given [`Block`].
    #[inline]
    #[cfg(not(feature = "std"))]
    #[expect(clippy::unused_self, reason = "Matches signature of `std` version")]
    fn block_shape(
        &mut self,
        block_id: GlobalId,
        storage: &BlockStorage,
        _: TypeId,
    ) -> &'static BlockShape<'static> {
        if let Some(block) = storage.get_block(block_id) {
            block.shape()
        } else {
            storage.get_block(GlobalId::new(0)).unwrap().shape()
        }
    }

    /// Get the index in the world cache for the given block position
    /// and cache side length.
    #[inline]
    #[must_use]
    #[expect(clippy::cast_sign_loss, reason = "Dimensions can only be as low as half `length`")]
    #[expect(clippy::cast_possible_truncation, reason = "Small Constant")]
    #[expect(clippy::cast_possible_wrap, reason = "Small Constant")]
    const fn cache_index(x: i32, y: i32, z: i32) -> usize {
        let half = Self::CACHE_SIDE_LENGTH as i32 / 2;
        let x = (x + half) as usize;
        let y = (y + half) as usize;
        let z = (z + half) as usize;

        (y * Self::CACHE_SIDE_LENGTH * Self::CACHE_SIDE_LENGTH) + (z * Self::CACHE_SIDE_LENGTH) + x
    }
}
