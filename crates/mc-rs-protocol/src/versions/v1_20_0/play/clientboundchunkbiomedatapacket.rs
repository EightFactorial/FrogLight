use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;

use crate::types::packets::biome_data::ChunkBiomeData;

// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, From, Into, Transcode)]
pub struct ClientboundChunkBiomeDataPacket(Vec<ChunkBiomeData>);
