use froglight_macros::FrogReadWrite;
use uuid::Uuid;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct PlayerListPacket {
    pub actions: u8,
    // TODO: Implement PlayerListData
    pub list_data: UnsizedBuffer,
}
