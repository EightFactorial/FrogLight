use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityAttributesS2CPacket {
    pub a: u32,
    pub b: Vec,
}
