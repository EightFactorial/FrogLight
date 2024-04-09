use froglight_macros::FrogReadWrite;

use crate::common::EntityUuid;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct PlayerRemoveS2CPacket {
    pub players: Vec<EntityUuid>,
}
