use mc_rs_macros::Transcode;
use uuid::Uuid;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundBossBarS2CPacket {
    pub a: Uuid,
    pub b: Enum,
}
