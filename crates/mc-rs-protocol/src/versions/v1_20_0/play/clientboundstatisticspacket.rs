// use hashbrown::HashMap;
use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

// TODO: Parse this packet
// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct ClientboundStatisticsPacket {
    pub stats: UnsizedByteBuffer,
}
