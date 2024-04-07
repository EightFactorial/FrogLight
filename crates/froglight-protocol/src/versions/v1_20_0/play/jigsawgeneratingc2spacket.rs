use froglight_macros::FrogReadWrite;

use crate::common::BlockPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct JigsawGeneratingC2SPacket {
    pub pos: BlockPosition,
    #[frog(var)]
    pub max_depth: u32,
    pub keep_jigsaws: bool,
}
