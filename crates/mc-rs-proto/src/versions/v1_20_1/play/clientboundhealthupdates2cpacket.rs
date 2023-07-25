use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundHealthUpdateS2CPacket {
    pub a: f32,
    pub b: u32,
    pub c: f32,
}
