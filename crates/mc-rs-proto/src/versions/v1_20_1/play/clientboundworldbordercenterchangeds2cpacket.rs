use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundWorldBorderCenterChangedS2CPacket {
    pub a: f64,
    pub b: f64,
}
