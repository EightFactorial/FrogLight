use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundOpenWrittenBookS2CPacket {
    pub a: Enum,
}
