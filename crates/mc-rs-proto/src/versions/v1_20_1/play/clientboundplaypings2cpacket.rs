use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundPlayPingS2CPacket {
    pub a: u32,
}
