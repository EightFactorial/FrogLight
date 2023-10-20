use mc_rs_macros::Transcode;

use crate::types::{packets::equipment::EntityEquipment, EntityId};

// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Transcode)]
// #[mctest(tests = ["transcode", "decode"], bytes = [8, ..])]
pub struct ClientboundEntityEquipmentUpdatePacket {
    pub entity_id: EntityId,
    pub equipment: EntityEquipment,
}
