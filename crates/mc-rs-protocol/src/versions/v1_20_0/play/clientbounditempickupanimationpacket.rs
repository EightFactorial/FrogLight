use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [32, 16, 64])]
pub struct ClientboundItemPickupAnimationPacket {
    pub item_entity_id: EntityId,
    pub collecter_entity_id: EntityId,
    #[var]
    pub count: u32,
}
