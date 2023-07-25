use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundPlayerSpawnPositionS2CPacket {
    pub a: BlockPos,
    pub b: f32,
}
