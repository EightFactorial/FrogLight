use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::common::ResourceKey;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 21, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 103, 101, 110, 101, 114, 105, 99, 95, 51, 120, 51, 5, 84, 105, 116, 108, 101])]
pub struct OpenScreenS2CPacket {
    #[frog(var)]
    pub container_id: u32,
    pub screen_handler_id: ResourceKey,
    pub name: CompactString,
}
