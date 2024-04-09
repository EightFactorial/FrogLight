use froglight_macros::FrogReadWrite;
use serde_json::Value;

use crate::common::UnsizedBuffer;

#[derive(Debug, Default, Clone, PartialEq, Eq, FrogReadWrite)]
pub struct ProfilelessChatMessageS2CPacket {
    pub message: Value,
    pub chat_type: UnsizedBuffer,
}
