use mc_rs_macros::Transcode;

use crate::types::packets::interaction::{BlockHitResult, InteractionHand};

#[derive(Debug, Clone, Copy, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1])]
pub struct ServerboundPlayerInteractBlockPacket {
    pub hand: InteractionHand,
    pub block_hit: BlockHitResult,
    #[var]
    pub sequence: u32,
}
