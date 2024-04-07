use froglight_macros::FrogReadWrite;

use crate::common::{EntityAnimation, EntityId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [42, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct EntityAnimationS2CPacket {
    pub id: EntityId,
    pub animation_id: EntityAnimation,
}
