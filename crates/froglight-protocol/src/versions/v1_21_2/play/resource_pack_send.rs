//! @generated by `froglight-generator` #e606248

use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ResourcePackSendPacket {
    pub field_0: CompactString,
    pub field_1: bool,
    pub field_2: Text,
}