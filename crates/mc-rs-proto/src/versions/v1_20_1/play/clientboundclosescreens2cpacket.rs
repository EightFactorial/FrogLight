use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundCloseScreenS2CPacket {
    pub a: u16,
}
