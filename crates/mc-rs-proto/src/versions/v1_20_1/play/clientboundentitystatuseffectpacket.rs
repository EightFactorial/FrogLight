use fastnbt::Value;
use mc_rs_macros::Transcode;

use crate::types::{EntityId, ResourceLocation};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityStatusEffectPacket {
    pub entity_id: EntityId,
    pub effect: ResourceLocation,
    pub amplifier: u8,
    #[var]
    pub duration: u32,
    pub flags: u8,
    pub data: Option<Value>,
}
