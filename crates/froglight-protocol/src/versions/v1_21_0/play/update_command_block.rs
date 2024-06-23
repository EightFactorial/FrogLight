use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::{
    common::BlockPosition,
    packet::{CommandBlockFlags, CommandBlockMode},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct UpdateCommandBlockPacket {
    pub position: BlockPosition,
    pub command: CompactString,
    pub mode: CommandBlockMode,
    pub flags: CommandBlockFlags,
}
