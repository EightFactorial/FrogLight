use froglight_macros::FrogReadWrite;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ServerMetadataPacket {
    // TODO: Text
    pub description: Value,
    pub favicon: Option<Vec<u8>>,
}
