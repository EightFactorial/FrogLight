use mc_rs_macros::Transcode;

use crate::types::enums::Difficulty;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundDifficultyPacket {
    pub difficulty: Difficulty,
    pub locked: bool,
}
