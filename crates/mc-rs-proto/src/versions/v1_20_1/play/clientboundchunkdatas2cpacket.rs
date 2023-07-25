use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundChunkDataS2CPacket {
    pub a: u32,
    pub b: u32,
}
