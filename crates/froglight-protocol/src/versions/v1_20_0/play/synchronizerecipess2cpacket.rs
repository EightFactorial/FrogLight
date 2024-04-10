use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct SynchronizeRecipesS2CPacket {
    // TODO: Implement recipes
    pub recipes: UnsizedBuffer,
}
