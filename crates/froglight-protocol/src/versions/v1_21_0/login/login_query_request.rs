use froglight_components::resourcekey::ResourceKey;
use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct LoginQueryRequestPacket {
    #[frog(var)]
    pub id: u32,
    pub identifier: ResourceKey,
    pub payload: UnsizedBuffer,
}
