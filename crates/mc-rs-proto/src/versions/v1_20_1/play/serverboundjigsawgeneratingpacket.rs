use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundJigsawGeneratingPacket {
    pub a: BlockPos,
    pub b: u32,
    pub c: bool,
}
