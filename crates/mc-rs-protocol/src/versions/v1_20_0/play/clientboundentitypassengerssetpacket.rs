use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [32, 5, 0, 1, 2, 3, 4])]
pub struct ClientboundEntityPassengersSetPacket {
    pub vehicle: EntityId,
    pub passengers: Vec<EntityId>,
}
