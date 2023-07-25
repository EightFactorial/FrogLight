use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundQueryPingC2SPacket {
    pub time: u64,
}
