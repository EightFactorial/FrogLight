use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

use super::KeepAliveS2CPacket;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite,
)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0, 0, 0, 0, 0, 0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct KeepAliveC2SPacket(pub u64);

impl From<KeepAliveS2CPacket> for KeepAliveC2SPacket {
    fn from(packet: KeepAliveS2CPacket) -> Self { Self(packet.0) }
}
