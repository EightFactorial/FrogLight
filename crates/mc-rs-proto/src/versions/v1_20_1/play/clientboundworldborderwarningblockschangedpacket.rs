use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundWorldBorderWarningBlocksChangedPacket {
    pub a: u32,
}
