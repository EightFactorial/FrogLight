use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundWorldBorderInterpolateSizeS2CPacket {
    pub a: f64,
    pub b: f64,
    pub c: u64,
}
