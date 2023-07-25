use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundTeamPacket {
    pub a: String,
    pub b: u8,
    pub c: Vec,
}
