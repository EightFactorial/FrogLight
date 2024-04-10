use froglight_macros::FrogReadWrite;

use crate::common::ResourceKey;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0])]
pub struct SelectAdvancementTabS2CPacket {
    pub tab_id: Option<ResourceKey>,
}
