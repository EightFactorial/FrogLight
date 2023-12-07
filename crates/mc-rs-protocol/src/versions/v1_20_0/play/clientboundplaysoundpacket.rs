use mc_rs_macros::Transcode;

use crate::types::{enums::SoundType, packets::sound::PacketSoundType};

// TODO: Create tests for this packet
#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct ClientboundPlaySoundPacket {
    pub data: PacketSoundType,
    pub kind: SoundType,
    pub pos_x: i32,
    pub pos_y: i32,
    pub pos_z: i32,
    pub volume: f32,
    pub pitch: f32,
    pub seed: u64,
}
