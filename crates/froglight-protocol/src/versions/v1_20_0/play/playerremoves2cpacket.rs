use froglight_macros::FrogReadWrite;

use crate::common::EntityUuid;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct PlayerRemoveS2CPacket(pub Vec<EntityUuid>);
