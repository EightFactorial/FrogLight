use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundHealthUpdateS2CPacket {
    pub a: f32,
    pub b: u32,
    pub c: f32,
}
