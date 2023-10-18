use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntitySetHeadYawPacket {
    pub entity_id: EntityId,
    pub yaw: i8,
}
