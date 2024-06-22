use froglight_macros::FrogReadWrite;

use crate::common::ResourceKey;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 13, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 97, 105, 114])]
pub struct CraftFailedResponsePacket {
    pub container_id: u8,
    pub recipe: ResourceKey,
}
