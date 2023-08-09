use mc_rs_macros::Transcode;

use crate::types::{
    packets::command_block::{CommandBlockFlags, CommandBlockMode},
    position::BlockPos,
};

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundUpdateCommandBlockPacket {
    pub position: BlockPos,
    pub command: String,
    pub mode: CommandBlockMode,
    pub flags: CommandBlockFlags,
}
