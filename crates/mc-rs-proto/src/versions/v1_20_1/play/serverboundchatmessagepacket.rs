use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundChatMessagePacket {
    pub a: String,
    pub b: u64,
    pub c: u64,
    pub d: Object,
}
