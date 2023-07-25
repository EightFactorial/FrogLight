use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundUpdateCommandBlockC2SPacket {
    pub a: BlockPos,
    pub b: String,
    pub c: Enum,
    pub d: u8,
}
