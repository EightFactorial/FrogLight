use mc_rs_macros::Transcode;

use crate::types::{enums::Direction, packets::player_action::PlayerAction, position::BlockPos};

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundPlayerActionPacket {
    pub action: PlayerAction,
    pub position: BlockPos,
    pub direction: Direction,
    #[var]
    pub sequence: u32,
}
