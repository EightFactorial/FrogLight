use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

use crate::common::ResourceKey;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [16, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 99, 111, 111, 107, 105, 101])]
pub struct CookieRequestPacket {
    pub cookie: ResourceKey,
}
