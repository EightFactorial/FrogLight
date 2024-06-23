use froglight_macros::FrogReadWrite;

use crate::common::ResourceKey;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct CraftRequestPacket {
    pub container_id: u8,
    pub recipe: ResourceKey,
    pub make_all: bool,
}
