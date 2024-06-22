//! @generated by `froglight-generator` #3ae6f0f

use compact_str::CompactString;
use froglight_macros::FrogReadWrite;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ChatMessageS2CPacket {
    pub field_0: Uuid,
    #[frog(var)]
    pub field_1: u32,
    pub field_2: [u8],
    pub field_3: Option<()>,
    pub field_4: CompactString,
    pub field_5: i64,
    pub field_6: i64,
    #[frog(var)]
    pub field_7: u32,
    pub field_8: [u8],
    pub field_9: Vec<()>,
    pub field_10: Option<()>,
    pub field_11: Enum,
    pub field_12: BitSet,
}
