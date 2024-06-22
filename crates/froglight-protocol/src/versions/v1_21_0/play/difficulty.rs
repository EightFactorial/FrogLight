use froglight_macros::FrogReadWrite;

use crate::common::Difficulty;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct DifficultyPacket {
    pub difficulty: Difficulty,
    pub locked: bool,
}
