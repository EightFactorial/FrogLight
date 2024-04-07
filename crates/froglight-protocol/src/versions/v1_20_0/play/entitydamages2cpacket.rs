use bevy_math::DVec3;
use froglight_macros::FrogReadWrite;

use crate::common::{EntityId, NonZero};

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [42, 0, 0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct EntityDamageS2CPacket {
    pub entity_id: EntityId,
    #[frog(var)]
    pub damage_type: u32,
    pub source_cause: NonZero<EntityId>,
    pub source_direct: NonZero<EntityId>,
    pub position: Option<DVec3>,
}
