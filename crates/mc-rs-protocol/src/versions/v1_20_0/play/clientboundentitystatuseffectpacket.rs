use azalea_nbt::Nbt;
use mc_rs_macros::Transcode;

use crate::types::{EntityId, ResourceLocation};

// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct ClientboundEntityStatusEffectPacket {
    pub entity_id: EntityId,
    pub effect: ResourceLocation,
    pub amplifier: u8,
    #[var]
    pub duration: u32,
    pub flags: u8,
    pub data: Option<Nbt>,
}
