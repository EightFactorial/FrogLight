use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundResourcePackSendS2CPacket {
    pub a: String,
    pub b: String,
    pub c: bool,
    pub d: Object,
}
