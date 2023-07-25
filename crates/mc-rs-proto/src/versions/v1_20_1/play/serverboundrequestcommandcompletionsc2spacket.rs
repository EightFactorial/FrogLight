use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundRequestCommandCompletionsC2SPacket {
    pub a: u32,
    pub b: String,
}
