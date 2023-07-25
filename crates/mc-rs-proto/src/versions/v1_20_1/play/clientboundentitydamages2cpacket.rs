use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityDamageS2CPacket {
    pub a: u32,
    pub b: u32,
    pub c: Option,
}
