use mc_rs_macros::Transcode;

use crate::types::{position::AngleData, EntityId};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityRotatePacket {
    pub entity_id: EntityId,
    pub angle: AngleData,
    pub on_ground: bool,
}
