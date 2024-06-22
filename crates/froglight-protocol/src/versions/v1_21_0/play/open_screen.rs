use froglight_components::resourcekey::ResourceKey;
use froglight_macros::FrogReadWrite;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 21, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 103, 101, 110, 101, 114, 105, 99, 95, 51, 120, 51, 5, 84, 105, 116, 108, 101])]
pub struct OpenScreenPacket {
    #[frog(var)]
    pub container_id: u32,
    pub handler: ResourceKey,
    // TODO: Text
    pub name: Value,
}
