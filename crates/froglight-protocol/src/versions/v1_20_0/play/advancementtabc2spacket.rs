use froglight_macros::FrogReadWrite;

use crate::{common::ResourceKey, packet::AdvancementTabAction};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct AdvancementTabC2SPacket {
    pub action: AdvancementTabAction,
    pub tab_to_open: Option<ResourceKey>,
}
