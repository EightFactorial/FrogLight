use mc_rs_macros::Transcode;

use crate::types::{packets::entity_data::EntityDataVec, EntityId};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityTrackerUpdatePacket {
    pub entity_id: EntityId,
    pub metadata: EntityDataVec,
}
