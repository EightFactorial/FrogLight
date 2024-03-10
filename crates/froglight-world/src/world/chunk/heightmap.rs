use std::io::Cursor;

use bevy_log::warn;
use bevy_reflect::Reflect;
use froglight_protocol::io::FrogRead;
use simdnbt::owned::Nbt;

use super::ChunkDecodeError;
use crate::world::container::HeightMapContainer;

/// A collection of heightmaps for a [`Chunk`](super::Chunk).
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct HeightMaps {
    /// The highest solid blocks
    ///
    /// This is used for blocking rain and snow particles.
    pub motion_blocking: HeightMapContainer,
    /// The highest non-air blocks
    ///
    /// This is used to detect if beacons beams are blocked.
    pub world_surface: HeightMapContainer,
}

impl HeightMaps {
    /// Decodes [`HeightMaps`] from a buffer.
    pub(crate) fn decode(height: usize, buf: &mut Cursor<&[u8]>) -> Result<Self, ChunkDecodeError> {
        let mut heightmaps = HeightMaps::default();

        // Read NBT from buffer
        if let Nbt::Some(base) = Nbt::fg_read(buf)? {
            let mut base = base.into_inner();

            // Read MOTION_BLOCKING
            if let Ok(Some(heightmap)) =
                HeightMapContainer::try_from_nbt(height, "MOTION_BLOCKING", &mut base)
            {
                heightmaps.motion_blocking = heightmap;
            } else {
                warn!("Chunk is missing `MOTION_BLOCKING` heightmap");
            }

            // Read WORLD_SURFACE
            if let Ok(Some(heightmap)) =
                HeightMapContainer::try_from_nbt(height, "WORLD_SURFACE", &mut base)
            {
                heightmaps.world_surface = heightmap;
            } else {
                warn!("Chunk is missing `WORLD_SURFACE` heightmap");
            }
        } else {
            warn!("Chunk contained no NBT data");
        }

        Ok(heightmaps)
    }
}

// TODO: Test heightmap decode
// #[test]
// fn test_heightmap_decode() {}
