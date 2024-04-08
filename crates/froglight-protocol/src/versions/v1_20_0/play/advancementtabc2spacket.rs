use froglight_macros::FrogReadWrite;

use crate::common::{AdvancementTabAction, ResourceKey};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct AdvancementTabC2SPacket {
    pub action: AdvancementTabAction,
    pub tab_to_open: Option<ResourceKey>,
}
