use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

// TODO: Parse this packet
// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, From, Into, Transcode)]
pub struct ClientboundCommandTreePacket(UnsizedByteBuffer);
