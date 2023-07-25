use mc_rs_macros::Transcode;
use uuid::Uuid;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundChatMessageS2CPacket {
    pub a: Uuid,
    pub b: u32,
    pub c: Object,
    pub d: Object,
}
