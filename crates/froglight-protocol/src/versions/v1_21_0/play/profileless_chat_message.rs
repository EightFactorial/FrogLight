use froglight_macros::FrogReadWrite;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ProfilelessChatMessagePacket {
    // TODO: Text
    pub message: Value,
    #[frog(var)]
    pub chat_type: u32,
    // TODO: Text
    pub sender: Value,
    // TODO: Text
    pub target: Option<Value>,
}
