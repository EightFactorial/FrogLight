use froglight_macros::FrogReadWrite;

use crate::common::BlockPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct QueryBlockNbtC2SPacket {
    #[frog(var)]
    pub transaction_id: u32,
    pub pos: BlockPosition,
}
