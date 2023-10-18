use mc_rs_macros::Transcode;

use crate::types::position::BlockPos;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundQueryBlockNbtPacket {
    #[var]
    pub id: u32,
    pub position: BlockPos,
}
