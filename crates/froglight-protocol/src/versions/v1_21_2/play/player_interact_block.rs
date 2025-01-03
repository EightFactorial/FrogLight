//! @generated by `froglight-generator` #8ddd9f0

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct PlayerInteractBlockPacket {
    pub field_0: Enum,
    pub field_1: BlockHitResult,
    #[frog(var)]
    pub field_2: u32,
}
