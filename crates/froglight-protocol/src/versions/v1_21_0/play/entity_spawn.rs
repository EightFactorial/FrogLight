use froglight_components::entity::{EntityId, EntityUuid};
use froglight_macros::FrogReadWrite;
use glam::{DVec3, I16Vec3};

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct EntitySpawnPacket {
    pub entity_id: EntityId,
    pub entity_uuid: EntityUuid,
    #[frog(var)]
    pub entity_type: u32,
    pub position: DVec3,
    pub pitch: u8,
    pub yaw: u8,
    pub head_yaw: u8,
    #[frog(var)]
    pub entity_data: u32,
    pub velocity: I16Vec3,
}
