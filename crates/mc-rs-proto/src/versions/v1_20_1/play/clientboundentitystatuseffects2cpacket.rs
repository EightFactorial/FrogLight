use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityStatusEffectS2CPacket {
    pub a: u32,
    pub b: Object,
    pub c: u8,
    pub d: u32,
    pub e: u8,
    pub f: Object,
}
