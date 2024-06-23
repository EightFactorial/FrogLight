use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct PlaySoundFromEntityPacket {
    // TODO: Implement SoundData
    pub data: UnsizedBuffer,
}
