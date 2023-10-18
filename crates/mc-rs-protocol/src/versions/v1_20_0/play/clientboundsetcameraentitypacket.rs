use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundSetCameraEntityPacket {
    pub entity_id: EntityId,
}
