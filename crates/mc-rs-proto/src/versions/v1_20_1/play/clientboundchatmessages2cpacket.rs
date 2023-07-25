use mc_rs_macros::Packet;
use uuid::Uuid;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundChatMessageS2CPacket {
    pub a: Uuid,
    pub b: u32,
    pub c: Object,
    pub d: Object,
}
