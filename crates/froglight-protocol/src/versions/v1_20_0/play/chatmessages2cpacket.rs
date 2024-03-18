use froglight_macros::FrogReadWrite;

use crate::common::{EntityUuid, UnsizedByteBuffer};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ChatMessageS2CPacket {
    pub sender: EntityUuid,
    #[frog(var)]
    pub index: u32,
    // TODO: Implement this
    pub data: UnsizedByteBuffer,
}
