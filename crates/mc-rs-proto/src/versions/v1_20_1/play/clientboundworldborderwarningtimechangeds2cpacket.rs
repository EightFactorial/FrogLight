use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundWorldBorderWarningTimeChangedS2CPacket {
    pub a: u32,
}
