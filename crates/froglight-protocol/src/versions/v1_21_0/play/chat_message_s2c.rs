use compact_str::CompactString;
use froglight_components::entity::EntityUuid;
use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ChatMessageS2CPacket {
    pub sender: EntityUuid,
    #[frog(var)]
    pub message_index: u32,
    pub signature: Option<[u8; 256]>,
    pub message: CompactString,
    pub timestamp: u64,
    pub salt: u64,
    // TODO: Implement ChatMessageData
    pub data: UnsizedBuffer,
}
