use froglight_macros::FrogReadWrite;

use crate::common::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0])]
pub struct DamageTiltS2CPacket {
    pub id: EntityId,
    pub yaw: f32,
}
