use froglight_macros::FrogReadWrite;

use crate::common::ResourceKey;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0])]
pub struct CraftFailedResponseS2CPacket {
    pub container_id: u8,
    pub recipe_id: ResourceKey,
}
