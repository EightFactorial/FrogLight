use mc_rs_macros::Packet;
use crate::types::ResourceLocation;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundStopSoundS2CPacket {
    pub a: u8,
    pub b: Enum,
    pub c: ResourceLocation,
}
