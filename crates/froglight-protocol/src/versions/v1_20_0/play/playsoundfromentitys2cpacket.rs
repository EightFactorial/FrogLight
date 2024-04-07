use froglight_macros::FrogReadWrite;

use crate::common::UnsizedByteBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct PlaySoundFromEntityS2CPacket {
    // TODO: Implement sound types and categories
    pub data: UnsizedByteBuffer,
    // pub sound: (),
    // pub category: (),
    // pub entity_id: EntityId,
    // pub volume: f32,
    // pub pitch: f32,
    // pub seed: u64,
}
