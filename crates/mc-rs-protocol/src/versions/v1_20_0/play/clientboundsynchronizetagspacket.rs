use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;

use crate::types::packets::tags::TagMap;

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, From, Into, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [1, 10, 109, 99, 45, 114, 115, 58, 116, 101, 115, 116, 0])]
pub struct ClientboundSynchronizeTagsPacket(TagMap);
