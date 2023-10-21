use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;

use super::serverboundplaypongpacket::ServerboundPlayPongPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref, DerefMut, From, Into, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 255])]
pub struct ClientboundPlayPingPacket(u32);

impl From<ServerboundPlayPongPacket> for ClientboundPlayPingPacket {
    fn from(value: ServerboundPlayPongPacket) -> Self { Self(value.into()) }
}
