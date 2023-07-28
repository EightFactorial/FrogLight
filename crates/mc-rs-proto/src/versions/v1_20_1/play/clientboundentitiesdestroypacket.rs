use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntitiesDestroyPacket {
    pub entity_ids: Vec<EntityId>,
}
