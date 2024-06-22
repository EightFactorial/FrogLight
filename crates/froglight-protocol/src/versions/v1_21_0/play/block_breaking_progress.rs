use froglight_components::entity::EntityId;
use froglight_macros::FrogReadWrite;

use crate::common::BlockPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct BlockBreakingProgressPacket {
    pub entity_id: EntityId,
    pub position: BlockPosition,
    pub state: u8,
}
