use froglight_macros::FrogReadWrite;

use crate::packet::AdvancementTabAction;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct AdvancementTabPacket {
    pub action: AdvancementTabAction,
}
