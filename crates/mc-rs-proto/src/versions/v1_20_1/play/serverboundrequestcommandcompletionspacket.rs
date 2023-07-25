use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundRequestCommandCompletionsPacket {
    pub a: u32,
    pub b: String,
}
