use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundWorldBorderWarningBlocksChangedS2CPacket {
    pub a: u32,
}
