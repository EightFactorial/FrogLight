use froglight_macros::FrogReadWrite;

use crate::common::{ResourceKey, UnsizedByteBuffer};

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct LoginQueryResponseC2SPacket {
    #[frog(var)]
    pub id: u32,
    pub identifier: ResourceKey,
    pub data: UnsizedByteBuffer,
}
