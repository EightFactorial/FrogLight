use mc_rs_macros::Transcode;
use uuid::Uuid;

use crate::types::{EntityId, ResourceLocation, Vec3};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntitySpawnPacket {
    pub entity_id: EntityId,
    pub uuid: Uuid,
    pub entity_kind: ResourceLocation,
    pub position: Vec3,
    pub yaw: i8,
    pub pitch: i8,
    pub head_yaw: i8,
    #[var]
    pub data: i32,
    pub x_vel: i16,
    pub y_vel: i16,
    pub z_vel: i16,
}
