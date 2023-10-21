use mc_rs_macros::Transcode;

use crate::types::{EntityId, ResourceLocation};

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [8, 5, 77, 67, 45, 82, 83])]
pub struct ClientboundRemoveEntityStatusEffectPacket {
    pub entity_id: EntityId,
    pub effect: ResourceLocation,
}
