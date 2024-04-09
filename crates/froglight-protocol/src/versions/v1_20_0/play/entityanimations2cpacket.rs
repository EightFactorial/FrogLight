use froglight_macros::FrogReadWrite;

use crate::{common::EntityId, packet::EntityAnimation};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [42, 0])]
pub struct EntityAnimationS2CPacket {
    pub id: EntityId,
    pub animation_id: EntityAnimation,
}
