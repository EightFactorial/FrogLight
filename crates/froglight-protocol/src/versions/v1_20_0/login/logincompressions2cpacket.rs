use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [128, 2])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct LoginCompressionS2CPacket(#[frog(var)] pub i32);
