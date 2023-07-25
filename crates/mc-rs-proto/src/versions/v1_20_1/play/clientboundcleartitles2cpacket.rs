use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundClearTitleS2CPacket {
    pub a: bool,
}
