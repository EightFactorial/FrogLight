use froglight_macros::FrogReadWrite;

use crate::common::{BlockPosition, EntityId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct BlockBreakingProgressS2CPacket {
    pub entity_id: EntityId,
    pub pos: BlockPosition,
    pub progress: u8,
}
