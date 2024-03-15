use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

use super::CommonPingS2CPacket;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite,
)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0, 0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct CommonPongC2SPacket(pub u32);

impl From<CommonPingS2CPacket> for CommonPongC2SPacket {
    fn from(packet: CommonPingS2CPacket) -> Self { Self(packet.0) }
}
