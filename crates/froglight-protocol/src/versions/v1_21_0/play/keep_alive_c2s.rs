use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite,
)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0, 0, 0, 0, 0, 0, 0, 0])]
pub struct KeepAliveC2SPacket {
    pub time: u64,
}

impl From<super::KeepAliveS2CPacket> for KeepAliveC2SPacket {
    fn from(packet: super::KeepAliveS2CPacket) -> Self { Self::from(packet.time) }
}
