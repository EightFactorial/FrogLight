use mc_rs_macros::Transcode;

use crate::types::{packets::attribute::EntityAttribute, EntityId};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityAttributesPacket {
    pub entity_id: EntityId,
    pub attributes: Vec<EntityAttribute>,
}
