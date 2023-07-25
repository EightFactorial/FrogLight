use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundCooldownUpdatePacket {
    pub a: Object,
    pub b: u32,
}
