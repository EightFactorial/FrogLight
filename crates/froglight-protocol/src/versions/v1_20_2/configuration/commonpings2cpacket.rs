use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

use super::CommonPongC2SPacket;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite,
)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0, 0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct CommonPingS2CPacket(pub u32);

impl From<CommonPongC2SPacket> for CommonPingS2CPacket {
    fn from(packet: CommonPongC2SPacket) -> Self { Self(packet.0) }
}
