use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundRemoveEntityStatusEffectS2CPacket {
    pub a: u32,
    pub b: Object,
}
