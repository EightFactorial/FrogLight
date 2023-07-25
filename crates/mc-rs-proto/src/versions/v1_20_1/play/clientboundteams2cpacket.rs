use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundTeamS2CPacket {
    pub a: String,
    pub b: u8,
    pub c: Vec,
}
