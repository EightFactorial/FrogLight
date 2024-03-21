use froglight_macros::FrogReadWrite;

use crate::common::{AdvancementTabAction, ResourceKey};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct AdvancementTabC2SPacket {
    pub action: AdvancementTabAction,
    pub tab_to_open: Option<ResourceKey>,
}
