use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundUpdateSignPacket {
    pub a: BlockPos,
    pub b: bool,
    pub c: String,
}
