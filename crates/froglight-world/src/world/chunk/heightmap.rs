use std::io::Cursor;

use froglight_protocol::io::FrogRead;
use simdnbt::owned::{Nbt, NbtTag};

use super::ChunkDecodeError;

/// A [`HeightMap`] is a 16x16 grid of height values.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct HeightMaps {
    pub motion_blocking: Vec<i64>,
    pub world_surface: Vec<i64>,
}

impl HeightMaps {
    pub(crate) fn decode(buf: &mut Cursor<&[u8]>) -> Result<Self, ChunkDecodeError> {
        let mut heightmaps = HeightMaps::default();

        if let Nbt::Some(base) = Nbt::frog_read(buf)? {
            let mut base = base.into_inner();

            if let Some(NbtTag::LongArray(motion_blocking)) = base.take("MOTION_BLOCKING") {
                heightmaps.motion_blocking = motion_blocking.clone();
            } else {
                #[cfg(feature = "logging")]
                bevy_log::warn!("Chunk is missing `MOTION_BLOCKING` heightmap");
            }

            if let Some(NbtTag::LongArray(world_surface)) = base.take("WORLD_SURFACE") {
                heightmaps.world_surface = world_surface.clone();
            } else {
                #[cfg(feature = "logging")]
                bevy_log::warn!("Chunk is missing `WORLD_SURFACE` heightmap");
            }
        } else {
            #[cfg(feature = "logging")]
            bevy_log::warn!("Chunk is missing heightmaps");
        }

        Ok(heightmaps)
    }
}

// TODO: Test heightmap decode
// #[test]
// fn test_heightmap_decode() {}
