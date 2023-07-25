use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerSpawnPositionPacket {
    pub a: BlockPos,
    pub b: f32,
}
