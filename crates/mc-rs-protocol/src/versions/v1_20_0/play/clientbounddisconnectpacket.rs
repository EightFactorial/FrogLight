use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;

// TODO: Verify that this is String and not FormattedText
// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, From, Into, Transcode)]
pub struct ClientboundDisconnectPacket(String);
