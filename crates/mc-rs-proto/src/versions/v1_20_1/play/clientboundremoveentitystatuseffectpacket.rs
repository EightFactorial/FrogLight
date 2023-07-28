use mc_rs_macros::Transcode;

use crate::types::{EntityId, ResourceLocation};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundRemoveEntityStatusEffectPacket {
    pub entity_id: EntityId,
    pub effect: ResourceLocation,
}
