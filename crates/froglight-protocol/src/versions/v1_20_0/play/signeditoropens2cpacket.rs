use froglight_macros::FrogReadWrite;

use crate::common::BlockPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 1])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct SignEditorOpenS2CPacket {
    pub pos: BlockPosition,
    pub front: bool,
}
