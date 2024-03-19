use froglight_macros::FrogReadWrite;

use crate::common::{EntityId, UnsizedByteBuffer};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [4, 0, 1, 153, 6, 1, 10, 0, 0, 3, 0, 6, 68, 97, 109, 97, 103, 101, 0, 0, 0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct EntityEquipmentUpdateS2CPacket {
    pub id: EntityId,
    // TODO: Implement entity equipment
    pub equipment: UnsizedByteBuffer,
}
