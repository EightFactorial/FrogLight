use froglight_common::EntityId;
use froglight_macros::FrogReadWrite;
use glam::I16Vec3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct EntityVelocityUpdatePacket {
    pub entity_id: EntityId,
    pub velocity: I16Vec3,
}
