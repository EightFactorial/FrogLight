use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityMoveRelativePacket {
    pub entity: EntityId,
    pub delta_x: u16,
    pub delta_y: u16,
    pub delta_z: u16,
    pub on_ground: bool,
}
