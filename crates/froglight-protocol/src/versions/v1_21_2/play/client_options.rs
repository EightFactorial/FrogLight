//! @generated by `froglight-generator` #e606248

use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ClientOptionsPacket {
    pub field_0: CompactString,
    pub field_1: u8,
    pub field_2: Enum,
    pub field_3: bool,
    pub field_4: u8,
    pub field_5: Enum,
    pub field_6: bool,
    pub field_7: bool,
    pub field_8: Enum,
}
