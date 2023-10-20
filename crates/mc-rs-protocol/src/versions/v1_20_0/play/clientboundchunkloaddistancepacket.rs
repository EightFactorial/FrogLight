use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref, DerefMut, From, Into, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [8])]
pub struct ClientboundChunkLoadDistancePacket(#[var] u32);

impl Default for ClientboundChunkLoadDistancePacket {
    fn default() -> Self { Self(8) }
}
