use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundRemoveEntityStatusEffectPacket {
    pub a: u32,
    pub b: Object,
}
