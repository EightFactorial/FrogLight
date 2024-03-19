use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [8])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct SimulationDistanceS2CPacket(#[frog(var)] pub u32);

impl Default for SimulationDistanceS2CPacket {
    fn default() -> Self { Self(8) }
}
