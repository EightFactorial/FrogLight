use froglight_macros::FrogReadWrite;

use crate::common::ResourceKey;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [13, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 97, 105, 114, 0])]
pub struct CooldownUpdateS2CPacket {
    pub item: ResourceKey,
    #[frog(var)]
    pub cooldown: u32,
}
