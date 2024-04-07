use froglight_macros::FrogReadWrite;

use crate::common::ResourceKey;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 10, 109, 99, 45, 114, 115, 58, 116, 101, 115, 116, 1])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct CraftRequestC2SPacket {
    pub container_id: u8,
    pub recipe: ResourceKey,
    pub craft_all: bool,
}
