use froglight_macros::FrogReadWrite;

use crate::common::UnsizedByteBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct SynchronizeRecipesS2CPacket {
    // TODO: Implement recipes
    pub recipes: UnsizedByteBuffer,
}
