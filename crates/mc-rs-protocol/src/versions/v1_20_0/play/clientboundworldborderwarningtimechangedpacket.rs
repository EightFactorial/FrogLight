use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundWorldBorderWarningTimeChangedPacket {
    #[var]
    pub warning_time: u32,
}
