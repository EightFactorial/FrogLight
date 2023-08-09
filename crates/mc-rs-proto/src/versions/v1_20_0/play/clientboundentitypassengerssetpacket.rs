use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityPassengersSetPacket {
    pub vehicle: EntityId,
    pub passengers: Vec<EntityId>,
}
