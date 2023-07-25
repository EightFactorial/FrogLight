use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundQueryBlockNbtC2SPacket {
    pub a: u32,
    pub b: BlockPos,
}
