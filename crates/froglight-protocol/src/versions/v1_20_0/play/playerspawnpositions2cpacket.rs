use froglight_macros::FrogReadWrite;

use crate::common::BlockPosition;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct PlayerSpawnPositionS2CPacket {
    pub pos: BlockPosition,
    pub angle: f32,
}
