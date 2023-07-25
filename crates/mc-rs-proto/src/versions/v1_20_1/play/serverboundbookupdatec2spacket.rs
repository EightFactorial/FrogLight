use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundBookUpdateC2SPacket {
    pub a: u32,
    pub b: Vec,
    pub c: Option,
}
