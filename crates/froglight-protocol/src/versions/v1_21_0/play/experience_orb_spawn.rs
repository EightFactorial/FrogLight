use froglight_components::entity::EntityId;
use froglight_macros::FrogReadWrite;
use glam::DVec3;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ExperienceOrbSpawnPacket {
    pub entity_id: EntityId,
    pub position: DVec3,
    pub experience: i16,
}
