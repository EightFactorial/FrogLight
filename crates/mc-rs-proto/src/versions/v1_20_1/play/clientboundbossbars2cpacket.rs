use mc_rs_macros::Packet;
use uuid::Uuid;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundBossBarS2CPacket {
    pub a: Uuid,
    pub b: Enum,
}
