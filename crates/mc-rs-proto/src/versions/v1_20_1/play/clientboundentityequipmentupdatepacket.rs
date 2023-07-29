use mc_rs_macros::Transcode;

use crate::types::{packets::equipment::EntityEquipment, EntityId};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityEquipmentUpdatePacket {
    pub entity_id: EntityId,
    pub equipment: EntityEquipment,
}
