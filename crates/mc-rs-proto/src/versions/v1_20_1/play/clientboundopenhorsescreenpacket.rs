use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundOpenHorseScreenPacket {
    pub a: u16,
    pub b: u32,
    pub c: u32,
}
