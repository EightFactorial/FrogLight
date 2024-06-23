use compact_str::CompactString;
use froglight_macros::FrogReadWrite;
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ResourcePackSendPacket {
    pub uuid: Uuid,
    pub url: CompactString,
    pub hash: CompactString,
    pub required: bool,
    // TODO: Text
    pub prompt: Option<Value>,
}
