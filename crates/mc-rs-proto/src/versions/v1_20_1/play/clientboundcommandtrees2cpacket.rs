use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundCommandTreeS2CPacket {
    pub a: Vec,
    pub b: u32,
}
