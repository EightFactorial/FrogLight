use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundWorldBorderWarningBlocksChangedPacket {
    #[var]
    pub warning_distance: u32,
}
