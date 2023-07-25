use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundLoginDisconnectS2CPacket {
    pub a: String,
}
