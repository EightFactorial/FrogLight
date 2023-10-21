use compact_str::CompactString;
use mc_rs_macros::Transcode;

use crate::types::{
    packets::command_block::{CommandBlockFlags, CommandBlockMode},
    position::BlockPos,
};

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ServerboundUpdateCommandBlockPacket {
    pub position: BlockPos,
    pub command: CompactString,
    pub mode: CommandBlockMode,
    pub flags: CommandBlockFlags,
}
