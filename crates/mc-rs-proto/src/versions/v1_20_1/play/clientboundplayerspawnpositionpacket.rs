use mc_rs_macros::Transcode;

use crate::types::position::BlockPos;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerSpawnPositionPacket {
    pub position: BlockPos,
    pub angle: f32,
}
