//! @generated by `froglight-generator` #73eaa37

use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct TeamPacket {
    pub field_0: CompactString,
    pub field_1: u8,
    pub field_2: u8,
    pub field_3: CompactString,
    pub field_4: CompactString,
    pub field_5: Enum,
    pub field_6: CompactString,
    pub field_7: Vec<()>,
}
