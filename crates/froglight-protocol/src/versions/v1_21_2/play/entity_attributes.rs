//! @generated by `froglight-generator` #8ddd9f0

use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct EntityAttributesPacket {
    #[frog(var)]
    pub field_0: u32,
    #[frog(var)]
    pub field_1: u32,
    pub field_2: f64,
    pub field_3: CompactString,
    pub field_4: f64,
    #[frog(var)]
    pub field_5: u32,
}
