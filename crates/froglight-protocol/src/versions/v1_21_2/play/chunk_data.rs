//! @generated by `froglight-generator` #e606248

use froglight_macros::FrogReadWrite;
use simdnbt::owned::Nbt;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ChunkDataPacket {
    pub field_0: i32,
    pub field_1: i32,
    pub field_2: Nbt,
    #[frog(var)]
    pub field_3: u32,
    pub field_4: [u8],
    pub field_5: BitSet,
    pub field_6: BitSet,
    pub field_7: BitSet,
    pub field_8: BitSet,
    pub field_9: Vec<()>,
    pub field_10: Vec<()>,
}