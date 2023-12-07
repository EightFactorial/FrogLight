use mc_rs_macros::Transcode;

use crate::types::{enums::SoundType, packets::sound::PacketSoundType, EntityId};

// TODO: Create tests for this packet
#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct ClientboundPlaySoundFromEntityPacket {
    pub data: PacketSoundType,
    pub kind: SoundType,
    pub entity_id: EntityId,
    pub volume: f32,
    pub pitch: f32,
    pub seed: u64,
}
