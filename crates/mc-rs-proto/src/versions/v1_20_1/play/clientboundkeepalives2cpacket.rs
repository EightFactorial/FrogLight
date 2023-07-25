use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundKeepAliveS2CPacket {
    pub a: u64,
}
