//! @generated by `froglight-generator` #ecfea09

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct RemoveMessagePacket {
    #[frog(var)]
    pub field_0: u32,
    pub field_1: [u8],
}
