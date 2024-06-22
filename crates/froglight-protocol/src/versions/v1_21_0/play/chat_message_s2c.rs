use froglight_components::entity::EntityUuid;
use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ChatMessageS2CPacket {
    pub sender: EntityUuid,
    #[frog(var)]
    pub index: u32,
    // TODO: Implement ChatMessageData
    pub message_data: UnsizedBuffer,
}
