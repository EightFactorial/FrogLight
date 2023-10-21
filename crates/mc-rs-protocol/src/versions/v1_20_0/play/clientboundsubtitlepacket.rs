use azalea_chat::FormattedText;
use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;

// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Deref, DerefMut, From, Into, Transcode)]
pub struct ClientboundSubtitlePacket(FormattedText);
