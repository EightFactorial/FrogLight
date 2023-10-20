use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [1, 1])]
pub struct ClientboundEntityStatusPacket {
    pub entity_id: EntityId,
    pub event_id: u8,
}
