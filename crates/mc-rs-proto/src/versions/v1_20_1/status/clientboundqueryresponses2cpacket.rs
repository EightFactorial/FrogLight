use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundQueryResponseS2CPacket {
    pub a: Object,
}
