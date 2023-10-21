use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref, DerefMut, From, Into, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [8])]
pub struct ClientboundSimulationDistancePacket(#[var] u32);

impl Default for ClientboundSimulationDistancePacket {
    fn default() -> Self { Self(8) }
}
