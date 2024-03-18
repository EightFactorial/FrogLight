use froglight_macros::FrogReadWrite;

use crate::common::Difficulty;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [2, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct DifficultyS2CPacket {
    pub difficulty: Difficulty,
    pub difficulty_locked: bool,
}
