use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct UnlockRecipesS2CPacket {
    // TODO: Read packet fields
    pub data: UnsizedBuffer,
}
