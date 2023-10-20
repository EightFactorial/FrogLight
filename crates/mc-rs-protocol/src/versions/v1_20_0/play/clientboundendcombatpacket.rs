use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deref, DerefMut, From, Into, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0, 0, 0, 0])]
pub struct ClientboundEndCombatPacket(u32);
