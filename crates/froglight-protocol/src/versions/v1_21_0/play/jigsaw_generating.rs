use froglight_macros::FrogReadWrite;

use crate::common::BlockPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct JigsawGeneratingPacket {
    pub position: BlockPosition,
    #[frog(var)]
    pub max_depth: u32,
    pub keep_jigsaws: bool,
}
