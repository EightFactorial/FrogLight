use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct PlaySoundS2CPacket {
    // TODO: Implement sound types and categories
    pub data: UnsizedBuffer,
    // pub sound: (),
    // pub category: (),
    // pub fixed_position: IVec3,
    // pub volume: f32,
    // pub pitch: f32,
    // pub seed: u64,
}
