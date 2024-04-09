use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct PlaySoundFromEntityS2CPacket {
    // TODO: Implement sound types and categories
    pub data: UnsizedBuffer,
    // pub sound: (),
    // pub category: (),
    // pub entity_id: EntityId,
    // pub volume: f32,
    // pub pitch: f32,
    // pub seed: u64,
}
