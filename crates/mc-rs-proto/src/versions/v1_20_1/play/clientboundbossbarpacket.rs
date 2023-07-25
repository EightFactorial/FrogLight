use mc_rs_macros::Transcode;
use uuid::Uuid;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundBossBarPacket {
    pub a: Uuid,
    pub b: Enum,
}
