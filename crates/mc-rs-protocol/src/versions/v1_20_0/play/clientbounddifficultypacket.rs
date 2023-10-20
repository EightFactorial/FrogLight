use mc_rs_macros::Transcode;

use crate::types::enums::Difficulty;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [2, 0])]
pub struct ClientboundDifficultyPacket {
    pub difficulty: Difficulty,
    pub locked: bool,
}
