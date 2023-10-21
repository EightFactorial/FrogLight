use compact_str::CompactString;
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 4, 116, 101, 115, 116])]
pub struct ServerboundRequestCommandCompletionsPacket {
    #[var]
    pub id: u32,
    pub command: CompactString,
}
