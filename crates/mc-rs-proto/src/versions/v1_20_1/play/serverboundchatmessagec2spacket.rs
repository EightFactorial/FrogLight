use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundChatMessageC2SPacket {
    pub a: String,
    pub b: u64,
    pub c: u64,
    pub d: Object,
}
