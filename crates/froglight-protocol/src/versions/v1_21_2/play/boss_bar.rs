//! @generated by `froglight-generator` #e606248

use froglight_macros::FrogReadWrite;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct BossBarPacket {
    pub field_0: Uuid,
    pub field_1: Enum,
}
