use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;

use super::serverboundkeepalivepacket::ServerboundKeepAlivePacket;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deref, DerefMut, From, Into, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ClientboundKeepAlivePacket(u64);

impl From<ServerboundKeepAlivePacket> for ClientboundKeepAlivePacket {
    fn from(value: ServerboundKeepAlivePacket) -> Self { Self(value.into()) }
}
