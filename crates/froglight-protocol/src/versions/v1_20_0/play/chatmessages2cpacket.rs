use froglight_macros::FrogReadWrite;

use crate::common::{EntityUuid, UnsizedBuffer};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct ChatMessageS2CPacket {
    pub sender: EntityUuid,
    #[frog(var)]
    pub index: u32,
    // TODO: Implement this
    pub data: UnsizedBuffer,
}
