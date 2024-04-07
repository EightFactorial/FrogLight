use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 255, 255])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct PlayPongC2SPacket(pub u32);
