use froglight_macros::FrogReadWrite;
use glam::{DVec3, I16Vec3};

use crate::common::{EntityId, EntityUuid};

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
pub struct EntitySpawnS2CPacket {
    pub id: EntityId,
    pub uuid: EntityUuid,
    #[frog(var)]
    pub entity_type: u32,
    pub position: DVec3,
    pub pitch: i8,
    pub yaw: i8,
    pub head_yaw: i8,
    #[frog(var)]
    pub entity_data: i32,
    pub velocity: I16Vec3,
}
