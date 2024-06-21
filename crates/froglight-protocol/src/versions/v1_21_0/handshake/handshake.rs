//! @generated by `froglight-generator` #ecfea09

use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct HandshakePacket {
    #[frog(var)]
    pub field_0: u32,
    pub field_1: CompactString,
    pub field_2: u16,
    #[frog(var)]
    pub field_3: u32,
}
