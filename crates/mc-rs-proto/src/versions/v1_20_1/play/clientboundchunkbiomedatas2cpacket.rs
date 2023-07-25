use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundChunkBiomeDataS2CPacket {
    pub a: Vec,
}
