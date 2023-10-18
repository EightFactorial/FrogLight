use mc_rs_macros::Transcode;

use crate::types::packets::interaction::{BlockHitResult, InteractionHand};

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundPlayerInteractBlockPacket {
    pub hand: InteractionHand,
    pub block_hit: BlockHitResult,
    #[var]
    pub sequence: u32,
}
