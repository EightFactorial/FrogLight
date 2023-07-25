use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundWorldTimeUpdateS2CPacket {
    pub a: u64,
    pub b: u64,
}
