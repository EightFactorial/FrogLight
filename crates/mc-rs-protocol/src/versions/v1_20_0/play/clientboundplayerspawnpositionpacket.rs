use mc_rs_macros::Transcode;

use crate::types::position::BlockPos;

#[derive(Debug, Default, Clone, Copy, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ClientboundPlayerSpawnPositionPacket {
    pub position: BlockPos,
    pub angle: f32,
}
