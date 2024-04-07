use froglight_macros::FrogReadWrite;

use crate::common::UnsizedByteBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct PlaySoundS2CPacket {
    // TODO: Implement sound types and categories
    pub data: UnsizedByteBuffer,
    // pub sound: (),
    // pub category: (),
    // pub fixed_position: IVec3,
    // pub volume: f32,
    // pub pitch: f32,
    // pub seed: u64,
}
