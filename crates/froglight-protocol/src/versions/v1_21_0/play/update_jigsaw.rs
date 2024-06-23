use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::common::{BlockPosition, ResourceKey};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct UpdateJigsawPacket {
    pub pos: BlockPosition,
    pub name: ResourceKey,
    pub target: ResourceKey,
    pub pool: ResourceKey,
    pub final_state: CompactString,
    pub joint_type: CompactString,
    #[frog(var)]
    pub selection_priority: u32,
    #[frog(var)]
    pub placement_priority: u32,
}
