use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundExperienceBarUpdateS2CPacket {
    pub a: f32,
    pub b: u32,
    pub c: u32,
}
