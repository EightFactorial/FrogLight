use froglight_macros::FrogReadWrite;
use uuid::Uuid;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct PlayerSessionPacket {
    pub session_uuid: Uuid,
    // TODO: Implement SessionData
    pub session_data: UnsizedBuffer,
}
