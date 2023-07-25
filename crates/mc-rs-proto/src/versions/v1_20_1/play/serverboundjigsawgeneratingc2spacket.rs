use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundJigsawGeneratingC2SPacket {
    pub a: BlockPos,
    pub b: u32,
    pub c: bool,
}
