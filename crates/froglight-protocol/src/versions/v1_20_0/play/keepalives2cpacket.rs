use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

use super::KeepAliveC2SPacket;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite,
)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0, 0, 0, 0, 0, 0, 0, 0])]
pub struct KeepAliveS2CPacket {
    pub value: u64,
}

impl From<KeepAliveC2SPacket> for KeepAliveS2CPacket {
    fn from(packet: KeepAliveC2SPacket) -> Self { Self::from(packet.value) }
}
