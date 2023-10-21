use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;

use super::clientboundplaypingpacket::ClientboundPlayPingPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref, DerefMut, From, Into, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 255, 255])]
pub struct ServerboundPlayPongPacket(u32);

impl From<ClientboundPlayPingPacket> for ServerboundPlayPongPacket {
    fn from(value: ClientboundPlayPingPacket) -> Self { Self(value.into()) }
}
