use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundChunkLoadDistanceS2CPacket {
    pub a: u32,
}
