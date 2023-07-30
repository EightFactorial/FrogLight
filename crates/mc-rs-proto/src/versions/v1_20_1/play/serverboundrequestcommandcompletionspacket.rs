use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundRequestCommandCompletionsPacket {
    #[var]
    pub id: u32,
    pub command: String,
}
