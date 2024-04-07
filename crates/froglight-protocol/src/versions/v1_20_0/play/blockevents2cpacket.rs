use froglight_macros::FrogReadWrite;

use crate::common::BlockPosition;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 1])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct BlockEventS2CPacket {
    pub pos: BlockPosition,
    pub kind: u8,
    pub data: u8,
    #[frog(var)]
    pub block: u32,
}
