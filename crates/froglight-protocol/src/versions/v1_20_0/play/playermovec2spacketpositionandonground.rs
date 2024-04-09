use froglight_macros::FrogReadWrite;
use glam::DVec3;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct PlayerMoveC2SPacketPositionAndOnGround {
    pub position: DVec3,
    pub on_ground: bool,
}
