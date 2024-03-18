use froglight_macros::FrogReadWrite;
use serde_json::Value;

use crate::common::UnsizedByteBuffer;

#[derive(Debug, Default, Clone, PartialEq, Eq, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ProfilelessChatMessageS2CPacket {
    pub message: Value,
    pub chat_type: UnsizedByteBuffer,
}
