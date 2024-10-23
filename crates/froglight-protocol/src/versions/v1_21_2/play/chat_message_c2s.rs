//! @generated by `froglight-generator` #e606248

use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ChatMessageC2SPacket {
    pub field_0: CompactString,
    pub field_1: i64,
    pub field_2: i64,
    pub field_3: [u8],
    pub field_4: Option<()>,
    #[frog(var)]
    pub field_5: u32,
    pub field_6: FixedBitSet,
}
