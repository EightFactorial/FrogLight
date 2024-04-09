use froglight_macros::FrogReadWrite;

use crate::common::ResourceKey;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0])]
pub struct CooldownUpdateS2CPacket {
    pub item: ResourceKey,
    #[frog(var)]
    pub cooldown: u32,
}
