use froglight_macros::FrogReadWrite;

use crate::common::Difficulty;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [2, 0])]
pub struct DifficultyS2CPacket {
    pub difficulty: Difficulty,
    pub difficulty_locked: bool,
}
