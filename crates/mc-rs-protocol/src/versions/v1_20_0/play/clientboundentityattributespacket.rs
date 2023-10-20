use mc_rs_macros::Transcode;

use crate::types::{packets::attribute::EntityAttribute, EntityId};

#[derive(Debug, Clone, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [42, 0])]
pub struct ClientboundEntityAttributesPacket {
    pub entity_id: EntityId,
    pub attributes: Vec<EntityAttribute>,
}
