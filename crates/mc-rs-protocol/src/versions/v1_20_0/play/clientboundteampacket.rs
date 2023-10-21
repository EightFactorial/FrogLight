use compact_str::CompactString;
use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

// TODO: Parse this packet
// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
pub struct ClientboundTeamPacket {
    pub name: CompactString,
    pub method: UnsizedByteBuffer,
}
