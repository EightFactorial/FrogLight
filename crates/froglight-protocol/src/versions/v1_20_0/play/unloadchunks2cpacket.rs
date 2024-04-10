use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

use crate::common::ChunkPosition;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite,
)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0, 0, 0, 0, 0, 0, 0, 0])]
pub struct UnloadChunkS2CPacket {
    pub position: ChunkPosition,
}
