use mc_rs_macros::Transcode;

use crate::types::{packets::equipment::EntityEquipment, EntityId};

// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [4, 0, 1, 153, 6, 1, 10, 0, 0, 3, 0, 6, 68, 97, 109, 97, 103, 101, 0, 0, 0, 0, 0])]
pub struct ClientboundEntityEquipmentUpdatePacket {
    pub entity_id: EntityId,
    pub equipment: EntityEquipment,
}
