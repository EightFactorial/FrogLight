use froglight_macros::FrogReadWrite;

use crate::{
    common::{BlockPosition, Direction},
    packet::ClientPlayerAction,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2])]
pub struct PlayerActionPacket {
    pub action: ClientPlayerAction,
    pub position: BlockPosition,
    pub direction: Direction,
    #[frog(var)]
    pub sequence: u32,
}
