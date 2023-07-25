use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundWorldBorderWarningTimeChangedPacket {
    pub a: u32,
}
