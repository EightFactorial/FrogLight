use bevy_math::Vec3;
use mc_rs_macros::Transcode;
use uuid::Uuid;

use crate::types::EntityId;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerSpawnPacket {
    pub entity_id: EntityId,
    pub uuid: Uuid,
    pub position: Vec3,
    pub yaw: i8,
    pub pitch: i8,
}
