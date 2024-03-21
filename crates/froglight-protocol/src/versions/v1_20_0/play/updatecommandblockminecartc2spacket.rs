use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::common::EntityId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 1])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct UpdateCommandBlockMinecartC2SPacket {
    pub entity_id: EntityId,
    pub command: CompactString,
    pub track_output: bool,
}
