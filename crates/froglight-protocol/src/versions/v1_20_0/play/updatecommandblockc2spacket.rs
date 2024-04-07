use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::common::{BlockPosition, CommandBlockFlags, CommandBlockMode};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct UpdateCommandBlockC2SPacket {
    pub position: BlockPosition,
    pub command: CompactString,
    pub mode: CommandBlockMode,
    pub flags: CommandBlockFlags,
}
