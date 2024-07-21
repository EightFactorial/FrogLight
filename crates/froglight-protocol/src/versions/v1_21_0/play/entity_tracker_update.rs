use froglight_common::EntityId;
use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [8, 0, 1, 128, 2, 255])]
pub struct EntityTrackerUpdatePacket {
    pub entity_id: EntityId,
    // TODO: Implement TrackerData
    pub tracker_data: UnsizedBuffer,
}
