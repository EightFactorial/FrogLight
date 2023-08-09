use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityRotateAndMoveRelativePacket {
    pub entity_id: EntityId,
    pub delta_x: u16,
    pub delta_y: u16,
    pub delta_z: u16,
    pub yaw: i8,
    pub pitch: i8,
    pub on_ground: bool,
}
