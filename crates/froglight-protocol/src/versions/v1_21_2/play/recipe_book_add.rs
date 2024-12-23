//! @generated by `froglight-generator` #8ddd9f0

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct RecipeBookAddPacket {
    #[frog(var)]
    pub field_0: u32,
    pub field_1: Option<VarInt>,
    pub field_2: u8,
    pub field_3: bool,
}
