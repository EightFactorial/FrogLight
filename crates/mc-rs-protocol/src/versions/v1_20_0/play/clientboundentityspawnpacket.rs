use mc_rs_macros::Transcode;
use uuid::Uuid;

use crate::types::{EntityId, Vec3};

// TODO: Create a test for this packet
#[derive(Debug, Clone, Copy, PartialEq, Transcode)]
pub struct ClientboundEntitySpawnPacket {
    pub entity_id: EntityId,
    pub uuid: Uuid,
    #[var]
    pub entity_kind: u32,
    pub position: Vec3,
    pub yaw: i8,
    pub pitch: i8,
    pub head_yaw: i8,
    #[var]
    pub data: i32,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}
