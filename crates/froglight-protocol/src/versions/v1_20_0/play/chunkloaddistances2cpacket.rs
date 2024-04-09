use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [8])]
pub struct ChunkLoadDistanceS2CPacket {
    #[frog(var)]
    pub distance: u32,
}

impl Default for ChunkLoadDistanceS2CPacket {
    fn default() -> Self { Self::from(8) }
}
