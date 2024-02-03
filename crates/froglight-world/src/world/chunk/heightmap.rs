use std::io::Cursor;

use bevy::reflect::Reflect;
use froglight_protocol::io::FrogRead;
use simdnbt::owned::{Nbt, NbtTag};

use super::ChunkDecodeError;
use crate::world::container::HeightMapContainer;

/// A [`HeightMap`] is a 16x16 grid of height values.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct HeightMaps {
    /// The highest solid blocks
    pub motion_blocking: HeightMapContainer,
    /// The highest non-air blocks
    pub world_surface: HeightMapContainer,
}

impl HeightMaps {
    /// Decodes [`HeightMaps`] from a buffer.
    pub(crate) fn decode(height: usize, buf: &mut Cursor<&[u8]>) -> Result<Self, ChunkDecodeError> {
        let mut heightmaps = HeightMaps::default();

        if let Nbt::Some(base) = Nbt::fg_read(buf)? {
            let mut base = base.into_inner();

            if let Some(NbtTag::LongArray(motion_blocking)) = base.take("MOTION_BLOCKING") {
                heightmaps.motion_blocking =
                    HeightMapContainer::try_from_vec(height, motion_blocking)?;
            } else {
                bevy::log::warn!("Chunk is missing `MOTION_BLOCKING` heightmap");
            }

            if let Some(NbtTag::LongArray(world_surface)) = base.take("WORLD_SURFACE") {
                heightmaps.world_surface = HeightMapContainer::try_from_vec(height, world_surface)?;
            } else {
                bevy::log::warn!("Chunk is missing `WORLD_SURFACE` heightmap");
            }
        } else {
            bevy::log::warn!("Chunk is missing heightmaps");
        }

        Ok(heightmaps)
    }
}

// TODO: Test heightmap decode
// #[test]
// fn test_heightmap_decode() {}
