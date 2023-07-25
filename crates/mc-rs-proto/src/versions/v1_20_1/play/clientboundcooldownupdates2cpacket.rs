use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundCooldownUpdateS2CPacket {
    pub a: Object,
    pub b: u32,
}
