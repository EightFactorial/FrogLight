use bevy_math::DVec3;
use froglight_macros::FrogReadWrite;

use crate::common::{LookAnchor, LookEntity};

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 32, 1])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct LookAtS2CPacket {
    pub self_anchor: LookAnchor,
    pub position: DVec3,
    pub entity: Option<LookEntity>,
}
