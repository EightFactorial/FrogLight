use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityStatusPacket {
    pub entity_id: EntityId,
    pub event_id: u8,
}
