use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityRotatePacket {
    pub entity_id: EntityId,
    pub yaw: i8,
    pub pitch: i8,
    pub on_ground: bool,
}
