use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundClientCommandPacket {
    pub a: u32,
    pub b: Enum,
    pub c: u32,
}
