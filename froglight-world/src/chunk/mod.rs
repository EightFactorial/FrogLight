//! TODO

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};

use crate::{component::ChunkBlockPos, prelude::*};

#[cfg(feature = "froglight-block")]
mod block;

/// A region of blocks in a world.
#[derive(Default, Clone)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Clone, Default, Component))]
pub struct Chunk;

impl Chunk {
    /// Get the height of this [`Chunk`].
    #[must_use]
    pub const fn height(&self) -> u32 { todo!() }

    /// Get the height offset of this [`Chunk`].
    #[must_use]
    pub const fn height_offset(&self) -> i32 { todo!() }

    /// Get a reference to the sections in this [`Chunk`].
    #[must_use]
    pub const fn sections(&self) -> &[()] { todo!() }

    /// Get a mutable reference to the sections in this [`Chunk`].
    #[must_use]
    pub const fn sections_mut(&mut self) -> &mut [()] { todo!() }

    /// Get the [`Block`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds.
    #[must_use]
    pub fn get_raw_block<P: Into<BlockPos>>(&self, position: P) -> Option<u32> {
        ChunkBlockPos::try_from_blockpos(position.into(), self.height_offset())
            .and_then(|pos| self.get_raw_block_pos::<ChunkBlockPos>(pos))
    }

    /// Get the [`Block`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds.
    #[must_use]
    pub fn get_raw_block_pos<P: Into<ChunkBlockPos>>(&self, _position: P) -> Option<u32> { todo!() }
}
