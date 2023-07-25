use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityAttributesPacket {
    pub a: u32,
    pub b: Vec,
}
