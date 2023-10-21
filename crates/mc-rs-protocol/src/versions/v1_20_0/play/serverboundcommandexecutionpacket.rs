use compact_str::CompactString;
use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [1, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ServerboundCommandExecutionPacket {
    pub command: CompactString,
    pub timestamp: u64,
    pub salt: u64,
    pub data: UnsizedByteBuffer,
}
