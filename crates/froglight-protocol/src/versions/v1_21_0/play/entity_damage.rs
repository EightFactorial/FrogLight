use froglight_common::EntityId;
use froglight_macros::FrogReadWrite;
use glam::DVec3;

use crate::common::NonZero;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [42, 0, 0, 0, 0])]
pub struct EntityDamagePacket {
    pub entity_id: EntityId,
    #[frog(var)]
    pub damage_type: u32,
    pub source_cause: NonZero<EntityId>,
    pub source_direct: NonZero<EntityId>,
    pub position: Option<DVec3>,
}
