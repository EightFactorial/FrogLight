use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundCommandTreePacket {
    pub a: Vec,
    pub b: u32,
}
