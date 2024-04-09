use froglight_macros::FrogReadWrite;

use crate::common::{EntityId, UnsizedBuffer};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [8, 0, 1, 128, 2, 255])]
pub struct EntityTrackerUpdateS2CPacket {
    pub entity_id: EntityId,
    // TODO: Implement metadata type
    pub metadata: UnsizedBuffer,
}
