use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

use crate::common::ChunkPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0])]
pub struct ChunkRenderDistanceCenterS2CPacket {
    #[frog(var)]
    pub chunk: ChunkPosition,
}
