use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

// TODO: Parse this packet
// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct ClientboundPlaySoundFromEntityPacket {
    pub data: UnsizedByteBuffer,
    // pub source: SoundSource,
    // #[var]
    // pub id: u32,
    // pub volume: f32,
    // pub pitch: f32,
    // pub seed: u64,
}
