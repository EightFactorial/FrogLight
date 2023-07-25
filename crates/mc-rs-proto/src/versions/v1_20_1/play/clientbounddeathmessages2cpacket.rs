use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundDeathMessageS2CPacket {
    pub a: u32,
    pub b: FormattedText,
}
