use mc_rs_macros::Transcode;

use crate::types::{packets::entity_data::EntityDataVec, EntityId};

#[derive(Debug, Clone, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [8, 0, 1, 128, 2, 255])]
pub struct ClientboundEntityTrackerUpdatePacket {
    pub entity_id: EntityId,
    pub metadata: EntityDataVec,
}
