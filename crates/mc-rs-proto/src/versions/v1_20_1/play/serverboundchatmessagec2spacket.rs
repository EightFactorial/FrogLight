use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundChatMessageC2SPacket {
    pub a: String,
    pub b: u64,
    pub c: u64,
    pub d: Object,
}
