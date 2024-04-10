use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

use crate::common::Difficulty;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [2])]
pub struct UpdateDifficultyC2SPacket {
    pub difficulty: Difficulty,
}
