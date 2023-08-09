use mc_rs_macros::Transcode;

use crate::types::{EntityId, UnsizedByteBuffer};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityTrackerUpdatePacket {
    pub entity_id: EntityId,
    pub data: UnsizedByteBuffer,
}
