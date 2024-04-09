use froglight_macros::FrogReadWrite;
use glam::DVec3;

use crate::packet::{LookAnchor, LookEntity};

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 32, 1])]
pub struct LookAtS2CPacket {
    pub self_anchor: LookAnchor,
    pub position: DVec3,
    pub entity: Option<LookEntity>,
}
