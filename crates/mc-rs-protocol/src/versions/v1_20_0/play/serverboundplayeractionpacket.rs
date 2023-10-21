use mc_rs_macros::Transcode;

use crate::types::{enums::Direction, packets::player_action::PlayerAction, position::BlockPos};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2])]
pub struct ServerboundPlayerActionPacket {
    pub action: PlayerAction,
    pub position: BlockPos,
    pub direction: Direction,
    #[var]
    pub sequence: u32,
}
