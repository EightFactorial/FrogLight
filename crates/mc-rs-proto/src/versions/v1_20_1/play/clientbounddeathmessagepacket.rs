use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundDeathMessagePacket {
    pub a: u32,
    pub b: FormattedText,
}
