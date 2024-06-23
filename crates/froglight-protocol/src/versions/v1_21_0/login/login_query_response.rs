use froglight_components::resourcekey::ResourceKey;
use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct LoginQueryResponsePacket {
    #[frog(var)]
    pub id: u32,
    pub identifier: ResourceKey,
    pub payload: Option<UnsizedBuffer>,
}
