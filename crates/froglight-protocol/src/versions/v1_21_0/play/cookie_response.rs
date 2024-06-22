use froglight_macros::FrogReadWrite;

use crate::common::{ResourceKey, UnsizedBuffer};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [16, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 99, 111, 111, 107, 105, 101, 4, 0, 1, 2, 3])]
pub struct CookieResponsePacket {
    pub cookie: ResourceKey,
    pub payload: Option<UnsizedBuffer>,
}
