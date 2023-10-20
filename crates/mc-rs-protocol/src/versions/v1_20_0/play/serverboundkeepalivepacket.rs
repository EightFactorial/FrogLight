use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;

use super::clientboundkeepalivepacket::ClientboundKeepAlivePacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref, DerefMut, From, Into, Transcode)]
pub struct ServerboundKeepAlivePacket(u64);

impl From<ClientboundKeepAlivePacket> for ServerboundKeepAlivePacket {
    fn from(packet: ClientboundKeepAlivePacket) -> Self { Self(packet.into()) }
}
