use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

use crate::packet::BiomeDataPacket;

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite,
)]
pub struct ChunkBiomeDataS2CPacket {
    pub data: Vec<BiomeDataPacket>,
}
