use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::common::{BlockPosition, ResourceKey};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct UpdateJigsawC2SPacket {
    pub pos: BlockPosition,
    pub name: ResourceKey,
    pub target: ResourceKey,
    pub pool: ResourceKey,
    pub final_state: CompactString,
    pub joint_type: CompactString,
}
