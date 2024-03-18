use froglight_macros::FrogReadWrite;

use crate::common::ChunkPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ChunkRenderDistanceCenterS2CPacket(#[frog(var)] pub ChunkPosition);
