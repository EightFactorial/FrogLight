//! TODO

use bevy_reflect::{FromType, PartialReflect};

#[cfg(all(feature = "async", feature = "block"))]
use crate::versioned::SharedChunk;
#[cfg(feature = "block")]
use crate::versioned::VersionedChunk;
use crate::{chunk::Chunk, prelude::BlockPosition};

/// A reflection attribute for [`Chunk`] and [`VersionedChunk`].
///
/// Allows for getting and setting raw block IDs in a chunk
/// without needing to know the specific type of chunk at compile time.
#[derive(Clone, Copy)]
pub struct ReflectChunk {
    get_block_fn: fn(&dyn PartialReflect, BlockPosition) -> Option<u32>,
    #[expect(clippy::type_complexity, reason = "No reason to break it apart when it's used once")]
    set_block_fn:
        fn(&mut dyn PartialReflect, u32, BlockPosition, &dyn Fn(u32) -> bool) -> Option<u32>,
}

impl ReflectChunk {
    /// Get the raw block ID at the given position in the chunk.
    ///
    /// Returns `None` if the position is out of bounds.
    #[inline]
    #[must_use]
    pub fn get_block_raw(&self, chunk: &Chunk, pos: BlockPosition) -> Option<u32> {
        (self.get_block_fn)(chunk, pos)
    }

    /// Set the raw block ID at the given position in the chunk.
    ///
    /// Returns the previous block ID,
    /// or `None` if the position is out of bounds.
    #[inline]
    pub fn set_block_raw(
        &self,
        chunk: &mut Chunk,
        block_id: u32,
        position: BlockPosition,
        is_air: &dyn Fn(u32) -> bool,
    ) -> Option<u32> {
        (self.set_block_fn)(chunk, block_id, position, is_air)
    }
}

// -------------------------------------------------------------------------------------------------

impl FromType<Chunk> for ReflectChunk {
    fn from_type() -> Self {
        Self {
            get_block_fn: |chunk, position| {
                if let Some(chunk) = chunk.try_downcast_ref::<Chunk>() {
                    chunk.get_block_raw(position)
                } else {
                    #[cfg(feature = "tracing")]
                    tracing::warn!("Attempted to get block from non-Chunk type!");
                    None
                }
            },
            set_block_fn: |chunk, block_id, position, is_air| {
                if let Some(chunk) = chunk.try_downcast_mut::<Chunk>() {
                    chunk.set_block_raw(block_id, position, is_air)
                } else {
                    #[cfg(feature = "tracing")]
                    tracing::warn!("Attempted to set block on non-Chunk type!");
                    None
                }
            },
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "block")]
impl FromType<VersionedChunk> for ReflectChunk {
    fn from_type() -> Self {
        Self {
            get_block_fn: |chunk, position| {
                if let Some(chunk) = chunk.try_downcast_ref::<VersionedChunk>() {
                    chunk.get_block_raw(position)
                } else {
                    #[cfg(feature = "tracing")]
                    tracing::warn!("Attempted to get block from non-VChunk type!");
                    None
                }
            },
            set_block_fn: |chunk, block_id, position, is_air| {
                if let Some(chunk) = chunk.try_downcast_mut::<VersionedChunk>() {
                    chunk.set_block_raw(block_id, position, is_air)
                } else {
                    #[cfg(feature = "tracing")]
                    tracing::warn!("Attempted to set block on non-VChunk type!");
                    None
                }
            },
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(all(feature = "async", feature = "block"))]
impl FromType<SharedChunk> for ReflectChunk {
    fn from_type() -> Self {
        Self {
            get_block_fn: |chunk, position| {
                if let Some(chunk) = chunk.try_downcast_ref::<SharedChunk>() {
                    chunk.read_blocking().get_block_raw(position)
                } else {
                    #[cfg(feature = "tracing")]
                    tracing::warn!("Attempted to get block from non-VChunk type!");
                    None
                }
            },
            set_block_fn: |chunk, block_id, position, is_air| {
                if let Some(chunk) = chunk.try_downcast_mut::<SharedChunk>() {
                    chunk.write_blocking().set_block_raw(block_id, position, is_air)
                } else {
                    #[cfg(feature = "tracing")]
                    tracing::warn!("Attempted to set block on non-VChunk type!");
                    None
                }
            },
        }
    }
}
