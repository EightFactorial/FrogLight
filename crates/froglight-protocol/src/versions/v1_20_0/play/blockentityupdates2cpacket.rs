use froglight_macros::FrogReadWrite;
use simdnbt::owned::Nbt;

use crate::common::{BlockPosition, ResourceKey};

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct BlockEntityUpdateS2CPacket {
    pub pos: BlockPosition,
    pub block_entity_type: ResourceKey,
    pub nbt: Nbt,
}
