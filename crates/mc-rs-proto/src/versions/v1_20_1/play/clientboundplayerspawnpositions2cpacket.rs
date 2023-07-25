use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerSpawnPositionS2CPacket {
    pub a: BlockPos,
    pub b: f32,
}
