use bevy_math::Vec3;
use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityPositionPacket {
    pub entity_id: EntityId,
    pub position: Vec3,
    pub yaw: i8,
    pub pitch: i8,
    pub on_ground: bool,
}
