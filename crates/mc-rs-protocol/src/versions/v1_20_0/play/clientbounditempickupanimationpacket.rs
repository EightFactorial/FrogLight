use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundItemPickupAnimationPacket {
    pub item_entity_id: EntityId,
    pub collecter_entity_id: EntityId,
    #[var]
    pub count: u32,
}
