use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlaySoundPacket {
    pub data: UnsizedByteBuffer,
    // pub b: Enum,
    // pub c: u32,
    // pub d: u32,
    // pub e: u32,
    // pub f: f32,
    // pub g: f32,
    // pub h: u64,
}
