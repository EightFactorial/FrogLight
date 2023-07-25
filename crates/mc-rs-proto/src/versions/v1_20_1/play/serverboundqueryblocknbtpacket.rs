use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundQueryBlockNbtPacket {
    pub a: u32,
    pub b: BlockPos,
}
