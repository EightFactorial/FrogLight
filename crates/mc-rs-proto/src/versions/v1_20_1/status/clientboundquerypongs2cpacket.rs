use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundQueryPongS2CPacket {
    pub a: u64,
}
