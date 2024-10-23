//! @generated by `froglight-generator` #e606248

use froglight_macros::FrogReadWrite;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct PlayerSessionPacket {
    pub field_0: Uuid,
    pub field_1: i64,
    pub field_2: PublicKey,
    pub field_3: Vec<u8>,
}